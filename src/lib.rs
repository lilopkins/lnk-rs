#![allow(unexpected_cfgs)]
#![warn(missing_docs)]

//! # Shell Link parser and writer for Rust.
//!
//! Works on any OS - although only really useful in Windows, this library can parse and write
//! .lnk files, a shell link, that can be understood by Windows.
//!
//! To get started, see the [ShellLink](struct.ShellLink.html) struct.
//!
//! The full specification of these files can be found at
//! [Microsoft's Website](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-shllink/16cb4ca1-9339-4d0c-a68d-bf1d6cc0f943).
//!
//! ## Read Example
//!
//! A simple example appears as follows:
//! ```
//! use lnk::ShellLink;
//! use lnk::encoding::WINDOWS_1252;
//! // ...
//! let shortcut = lnk::ShellLink::open("tests/data/test.lnk", WINDOWS_1252).unwrap();
//! println!("{:#?}", shortcut);
//! ```
//!
//! ## Write Example
//!
//! A simple example appears as follows:
//! ```ignore
//! use lnk::ShellLink;
//! // ...
//! ShellLink::new_simple(std::path::Path::new(r"C:\Windows\System32\notepad.exe"));
//! ```
//!
//! > **IMPORTANT!**: Writing capability is currently in a very early stage and probably won't work!

use binrw::BinReaderExt;
use getset::{Getters, MutGetters};
#[allow(unused)]
use log::{debug, error, info, trace, warn};
#[cfg(feature = "serde")]
use serde::Serialize;

use std::io::BufReader;
#[cfg(feature = "binwrite")]
use std::io::BufWriter;
use std::path::Path;
use std::{fs::File, io::Seek};

mod header;
pub use header::{
    FileAttributeFlags, HotkeyFlags, HotkeyKey, HotkeyModifiers, LinkFlags, ShellLinkHeader,
    ShowCommand,
};

/// The LinkTargetIDList structure specifies the target of the link. The presence of this optional
/// structure is specified by the HasLinkTargetIDList bit (LinkFlagssection 2.1.1) in the
/// ShellLinkHeader(section2.1).
pub mod linktarget;
pub use linktarget::LinkTargetIdList;

/// The LinkInfo structure specifies information necessary to resolve a
/// linktarget if it is not found in its original location. This includes
/// information about the volume that the target was stored on, the mapped
/// drive letter, and a Universal Naming Convention (UNC)form of the path
/// if one existed when the linkwas created. For more details about UNC
/// paths, see [MS-DFSNM] section 2.2.1.4
pub mod linkinfo;
pub use linkinfo::LinkInfo;

mod stringdata;
pub use stringdata::StringData;

/// Structures from the ExtraData section of the Shell Link.
pub mod extradata;
pub use extradata::ExtraData;

mod generic_types;
pub use generic_types::filetime::FileTime;
pub use generic_types::guid::*;
pub use generic_types::idlist::*;

mod current_offset;
pub use current_offset::*;

mod strings;
pub use strings::*;

mod itemid;
pub use itemid::*;

#[macro_use]
mod binread_flags;

mod error;
pub use error::Error;

/// A shell link
#[derive(Debug, Getters, MutGetters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[getset(get = "pub", get_mut = "pub")]
pub struct ShellLink {
    /// returns the [`ShellLinkHeader`] structure
    header: header::ShellLinkHeader,

    /// returns the [`LinkTargetIdList`] structure
    #[cfg_attr(feature = "serde", serde(skip))]
    linktarget_id_list: Option<linktarget::LinkTargetIdList>,

    /// returns the [`LinkInfo`] structure
    link_info: Option<linkinfo::LinkInfo>,

    /// returns the [`StringData`] structure
    string_data: StringData,

    /// returns the [`ExtraData`] structure
    #[allow(unused)]
    extra_data: extradata::ExtraData,

    /// encoding used for this link
    #[serde(skip)]
    #[getset(skip)]
    encoding: &'static encoding_rs::Encoding,
}

impl Default for ShellLink {
    /// Create a new ShellLink, left blank for manual configuration.
    /// For those who are not familar with the Shell Link specification, I
    /// suggest you look at the [`ShellLink::new_simple`] method.
    fn default() -> Self {
        let header = header::ShellLinkHeader::default();
        let encoding = if header.link_flags().contains(LinkFlags::IS_UNICODE) {
            encoding_rs::UTF_16LE
        } else {
            encoding_rs::WINDOWS_1252
        };
        Self {
            header,
            linktarget_id_list: None,
            link_info: None,
            string_data: Default::default(),
            extra_data: Default::default(),
            encoding,
        }
    }
}

impl ShellLink {
    /// Create a new ShellLink pointing to a location, with otherwise default settings.
    pub fn new_simple<P: AsRef<Path>>(to: P) -> std::io::Result<Self> {
        use std::fs;
        use std::path::PathBuf;

        let meta = fs::metadata(&to)?;
        let mut canonical = fs::canonicalize(&to)?.into_boxed_path();
        if cfg!(windows) {
            // Remove symbol for long path if present.
            let can_os = canonical.as_os_str().to_str().unwrap();
            if let Some(stripped) = can_os.strip_prefix("\\\\?\\") {
                canonical = PathBuf::new().join(stripped).into_boxed_path();
            }
        }

        let mut sl = Self::default();

        if meta.is_dir() {
            sl.header_mut()
                .set_file_attributes(FileAttributeFlags::FILE_ATTRIBUTE_DIRECTORY);
        } else {
            sl.set_relative_path(Some(format!(
                ".\\{}",
                canonical.file_name().unwrap().to_str().unwrap()
            )));
            sl.set_working_dir(Some(
                canonical.parent().unwrap().to_str().unwrap().to_string(),
            ));
        }

        Ok(sl)
    }

    /// change the encoding for this link
    pub fn with_encoding(mut self, encoding: &StringEncoding) -> Self {
        match encoding {
            StringEncoding::Unicode => {
                self.header
                    .link_flags_mut()
                    .set(LinkFlags::IS_UNICODE, true);
                self.encoding = encoding_rs::UTF_16LE;
            }
            StringEncoding::CodePage(cp) => {
                self.header
                    .link_flags_mut()
                    .set(LinkFlags::IS_UNICODE, false);
                self.encoding = cp;
            }
        }
        self
    }

    /// Save a shell link.
    ///
    /// Note that this doesn't save any [`ExtraData`](struct.ExtraData.html) entries.
    #[cfg(feature = "binwrite")]
    #[cfg_attr(feature = "binwrite", stability::unstable(feature = "save"))]
    pub fn save<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), Error> {
        use binrw::BinWrite;

        let mut w = BufWriter::new(File::create(path)?);

        debug!("Writing header...");
        // Invoke binwrite
        self.header()
            .write_le(&mut w)
            .map_err(|be| Error::while_writing("Header", be))?;

        let link_flags = *self.header().link_flags();

        debug!("Writing StringData...");
        self.string_data
            .write_le_args(&mut w, (link_flags, self.encoding))
            .map_err(|be| Error::while_writing("StringData", be))?;

        // if link_flags.contains(LinkFlags::HAS_LINK_TARGET_ID_LIST) {
        //     if let None = self.linktarget_id_list {
        //         error!("LinkTargetIDList not specified but expected!")
        //     }
        //     debug!("A LinkTargetIDList is marked as present. Writing.");
        //     let mut data: Vec<u8> = self.linktarget_id_list.clone().unwrap().into();
        //     w.write_all(&mut data)?;
        // }

        // if link_flags.contains(LinkFlags::HAS_LINK_INFO) {
        //     if let None = self.link_info {
        //         error!("LinkInfo not specified but expected!")
        //     }
        //     debug!("LinkInfo is marked as present. Writing.");
        //     let mut data: Vec<u8> = self.link_info.clone().unwrap().into();
        //     w.write_all(&mut data)?;
        // }

        // if link_flags.contains(LinkFlags::HAS_NAME) {
        //     if self.name_string == None {
        //         error!("Name not specified but expected!")
        //     }
        //     debug!("Name is marked as present. Writing.");
        //     w.write_all(&stringdata::to_data(
        //         self.name_string.as_ref().unwrap(),
        //         link_flags,
        //     ))?;
        // }

        // if link_flags.contains(LinkFlags::HAS_RELATIVE_PATH) {
        //     if self.relative_path == None {
        //         error!("Relative path not specified but expected!")
        //     }
        //     debug!("Relative path is marked as present. Writing.");
        //     w.write_all(&stringdata::to_data(
        //         self.relative_path.as_ref().unwrap(),
        //         link_flags,
        //     ))?;
        // }

        // if link_flags.contains(LinkFlags::HAS_WORKING_DIR) {
        //     if self.working_dir == None {
        //         error!("Working Directory not specified but expected!")
        //     }
        //     debug!("Working dir is marked as present. Writing.");
        //     w.write_all(&stringdata::to_data(
        //         self.working_dir.as_ref().unwrap(),
        //         link_flags,
        //     ))?;
        // }

        // if link_flags.contains(LinkFlags::HAS_ARGUMENTS) {
        //     if self.icon_location == None {
        //         error!("Arguments not specified but expected!")
        //     }
        //     debug!("Arguments are marked as present. Writing.");
        //     w.write_all(&stringdata::to_data(
        //         self.command_line_arguments.as_ref().unwrap(),
        //         link_flags,
        //     ))?;
        // }

        // if link_flags.contains(LinkFlags::HAS_ICON_LOCATION) {
        //     if self.icon_location == None {
        //         error!("Icon Location not specified but expected!")
        //     }
        //     debug!("Icon Location is marked as present. Writing.");
        //     w.write_all(&stringdata::to_data(
        //         self.icon_location.as_ref().unwrap(),
        //         link_flags,
        //     ))?;
        // }

        Ok(())
    }

    /// Open and parse a shell link
    ///
    /// All string which are stored in the `lnk` file are encoded with either
    /// Unicode (UTF-16LE) of any of the Windows code pages. Which of both is
    /// being used is specified by the [`LinkFlags::IS_UNICODE`] flag. Microsoft
    /// documents this as follows:
    ///
    /// > If this bit is set, the StringData section contains Unicode-encoded
    /// > strings; otherwise, it contains strings that are encoded using the
    /// > system default code page.
    /// >
    /// > (<https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-shllink/ae350202-3ba9-4790-9e9e-98935f4ee5af>)
    ///
    /// The system default code page is stored in
    /// `HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Nls\CodePage\ACP`
    ///
    /// Because we do not know what the system default code page was, you must
    /// specify this using the `encoding` parameter (see below). If you you do
    /// not know the system default code page either, you're lost. There is no
    /// way to correctly guess the used code page from the data in the `lnk`
    /// file.
    ///
    /// * `path` - path of the `lnk` file to be analyzed
    /// * `encoding` - character encoding to be used if the `lnk` file is not
    ///   Unicode encoded
    pub fn open<P: AsRef<std::path::Path>>(
        path: P,
        encoding: crate::strings::Encoding,
    ) -> Result<Self, Error> {
        debug!("Opening {:?}", path.as_ref());
        let mut reader = BufReader::new(File::open(path)?);
        trace!("Reading file.");

        let shell_link_header: ShellLinkHeader = reader
            .read_le()
            .map_err(|be| Error::while_parsing("ShellLinkHeader", be))?;
        debug!("Shell header: {:#?}", shell_link_header);

        let mut linktarget_id_list = None;
        let link_flags = *shell_link_header.link_flags();
        if link_flags.contains(LinkFlags::HAS_LINK_TARGET_ID_LIST) {
            debug!(
                "A LinkTargetIDList is marked as present. Parsing now at position 0x{:0x}",
                reader.stream_position()?
            );
            let list: LinkTargetIdList = reader
                .read_le()
                .map_err(|be| Error::while_parsing("LinkTargetIdList", be))?;
            debug!("LinkTargetIDList: {:#?}", list);
            linktarget_id_list = Some(list);
        }

        let mut link_info = None;
        if link_flags.contains(LinkFlags::HAS_LINK_INFO) {
            let link_info_offset = reader.stream_position().unwrap();
            debug!(
                "LinkInfo is marked as present. Parsing now at position 0x{:0x}",
                reader.stream_position().unwrap()
            );
            let info: LinkInfo = reader
                .read_le_args((encoding,))
                .map_err(|be| Error::while_parsing("LinkInfo", be))?;
            debug!("{:#?}", info);
            debug_assert_eq!(
                reader.stream_position().unwrap(),
                link_info_offset + u64::from(*(info.link_info_size()))
            );
            link_info = Some(info);
        }

        debug!(
            "reading StringData at 0x{:08x}",
            reader.stream_position().unwrap()
        );
        let string_data: StringData = reader
            .read_le_args((link_flags, encoding))
            .map_err(|be| Error::while_parsing("StringData", be))?;
        debug!("{:#?}", string_data);

        debug!(
            "reading ExtraData at 0x{:08x}",
            reader.stream_position().unwrap()
        );
        let extra_data: ExtraData = reader
            .read_le_args((encoding,))
            .map_err(|be| Error::while_parsing("ExtraData", be))?;

        let encoding = if shell_link_header
            .link_flags()
            .contains(LinkFlags::IS_UNICODE)
        {
            encoding_rs::UTF_16LE
        } else {
            encoding
        };

        Ok(Self {
            header: shell_link_header,
            linktarget_id_list,
            link_info,
            string_data,
            extra_data,
            encoding,
        })
    }

    /// returns the full path of the link target. This information
    /// is constructed completely from the LINK_INFO structure. So,
    /// if the lnk file does not contain such a structure, the result
    /// of this method will be `None`
    pub fn link_target(&self) -> Option<String> {
        if let Some(info) = self.link_info().as_ref() {
            let mut base_path = if info
                .link_info_flags()
                .has_common_network_relative_link_and_path_suffix()
            {
                info.common_network_relative_link()
                    .as_ref()
                    .expect("missing common network relative link")
                    .name()
            } else {
                info.local_base_path_unicode()
                    .as_ref()
                    .map(|s| &s[..])
                    .or(info.local_base_path())
                    .expect("missing local base path")
                    .to_string()
            };

            let common_path = info
                .common_path_suffix_unicode()
                .as_ref()
                .map(|s| &s[..])
                .unwrap_or(info.common_path_suffix());

            // join base_path and common_path;
            // make sure they're divided by exactly one '\' character.
            // if common_path is empty, there's nothing to join.
            if !common_path.is_empty() {
                if !base_path.ends_with('\\') {
                    base_path.push('\\');
                }
                base_path.push_str(common_path);
            }
            Some(base_path)
        } else {
            None
        }
    }

    /// Set the shell link's name
    pub fn set_name(&mut self, name: Option<String>) {
        self.header_mut()
            .update_link_flags(LinkFlags::HAS_NAME, name.is_some());
        self.string_data_mut().set_name_string(name);
    }

    /// Set the shell link's relative path
    pub fn set_relative_path(&mut self, relative_path: Option<String>) {
        self.header_mut()
            .update_link_flags(LinkFlags::HAS_RELATIVE_PATH, relative_path.is_some());
        self.string_data_mut().set_relative_path(relative_path);
    }

    /// Set the shell link's working directory
    pub fn set_working_dir(&mut self, working_dir: Option<String>) {
        self.header_mut()
            .update_link_flags(LinkFlags::HAS_WORKING_DIR, working_dir.is_some());
        self.string_data_mut().set_working_dir(working_dir);
    }

    /// Set the shell link's arguments
    pub fn set_arguments(&mut self, arguments: Option<String>) {
        self.header_mut()
            .update_link_flags(LinkFlags::HAS_ARGUMENTS, arguments.is_some());
        self.string_data_mut().set_command_line_arguments(arguments);
    }

    /// Set the shell link's icon location
    pub fn set_icon_location(&mut self, icon_location: Option<String>) {
        self.header_mut()
            .update_link_flags(LinkFlags::HAS_ICON_LOCATION, icon_location.is_some());
        self.string_data_mut().set_icon_location(icon_location);
    }
}
