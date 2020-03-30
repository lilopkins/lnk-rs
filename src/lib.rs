#![warn(missing_docs)]

//! # Shell Link parser and writer for Rust.
//! Works on any OS - although only really useful in Windows, this library can parse and write
//! .lnk files, a shell link, that can be understood by Windows.
//! 
//! To get started, see the [ShellLink](struct.ShellLink.html) struct.
//!
//! The full specification of these files can be found at
//! [Microsoft's Website](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-shllink/16cb4ca1-9339-4d0c-a68d-bf1d6cc0f943).
//!
//! ## Example
//! A simple example appears as follows:
//! ```rust
//! use lnk::ShellLink;
//! // ...
//! ShellLink::new_simple(std::path::Path::new(r"C:\Windows\System32\notepad.exe"));
//! ```

use byteorder::{ByteOrder, LE};
#[allow(unused)]
use log::{trace, debug, info, warn, error};

use std::io::{prelude::*, BufReader, BufWriter};
use std::fs::File;
use std::path::Path;

mod header;
pub use header::{
    ShellLinkHeader,
    LinkFlags,
    FileAttributeFlags,
    HotkeyFlags,
    HotkeyKey,
    HotkeyModifiers,
    ShowCommand
};

mod linktarget;
pub use linktarget::LinkTargetIdList;

mod linkinfo;
pub use linkinfo::LinkInfo;

mod stringdata;

mod extradata;
pub use extradata::ExtraData;

/// A shell link
#[derive(Clone, Debug)]
pub struct ShellLink {
    shell_link_header: header::ShellLinkHeader,
    linktarget_id_list: Option<linktarget::LinkTargetIdList>,
    link_info: Option<linkinfo::LinkInfo>,
    name_string: Option<String>,
    relative_path: Option<String>,
    working_dir: Option<String>,
    command_line_arguments: Option<String>,
    icon_location: Option<String>,
    extra_data: Vec<extradata::ExtraData>,
}

impl Default for ShellLink {
    /// Create a new ShellLink, left blank for manual configuration.
    /// For those who are not familar with the Shell Link specification, I
    /// suggest you look at the [`new_simple`](#method.new_simple) method.
    fn default() -> Self {
        Self {
            shell_link_header: header::ShellLinkHeader::default(),
            linktarget_id_list: None,
            link_info: None,
            name_string: None,
            relative_path: None,
            working_dir: None,
            command_line_arguments: None,
            icon_location: None,
            extra_data: vec![],
        }
    }
}

impl ShellLink {

    /// Create a new ShellLink pointing to a location, with otherwise default settings.
    pub fn new_simple<P: AsRef<Path>>(to: P) -> std::io::Result<Self> {
        use std::fs;

        let meta = fs::metadata(&to)?;
        let canonical = fs::canonicalize(to)?.into_boxed_path();

        let mut sl = Self::default();

        let mut flags = LinkFlags::IS_UNICODE;
        if meta.is_dir() {
            sl.header_mut().set_file_attributes(FileAttributeFlags::FILE_ATTRIBUTE_DIRECTORY);
        } else {
            flags |= LinkFlags::HAS_WORKING_DIR | LinkFlags::HAS_RELATIVE_PATH;
            let mut ances = canonical.ancestors();
            sl.set_relative_path(Some(format!("./{}", canonical.file_name().unwrap().to_str().unwrap())));
            sl.set_working_dir(Some(ances.next().unwrap().to_str().unwrap().to_string()));
            sl.header_mut().set_file_size(meta.len() as u32);
        }
        sl.header_mut().set_link_flags(flags);

        Ok(sl)
    }

    /// Save a shell link.
    /// 
    /// Note that this doesn't save any [`ExtraData`](struct.ExtraData.html) entries.
    pub fn save<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<()> {
        let mut w = BufWriter::new(File::create(path)?);

        debug!("Writing header...");
        let header_data: [u8; 0x4c] = self.shell_link_header.into();
        w.write_all(&header_data)?;

        if *self.header().link_flags() & LinkFlags::HAS_LINK_TARGET_ID_LIST
            == LinkFlags::HAS_LINK_TARGET_ID_LIST {


            if let None = self.linktarget_id_list { error!("LinkTargetIDList not specified but expected!") }
            debug!("A LinkTargetIDList is marked as present. Writing.");
            let mut data: Vec<u8> = self.linktarget_id_list.clone().unwrap().into();
            w.write_all(&mut data)?;
        }

        if *self.header().link_flags() & LinkFlags::HAS_LINK_INFO
            == LinkFlags::HAS_LINK_INFO {

            if let None = self.link_info { error!("LinkInfo not specified but expected!") }
            debug!("LinkInfo is marked as present. Writing.");
            let mut data: Vec<u8> = self.link_info.clone().unwrap().into();
            w.write_all(&mut data)?;
        }

        if *self.header().link_flags() & LinkFlags::HAS_NAME
            == LinkFlags::HAS_NAME {

            if self.name_string == None { error!("Name not specified but expected!") }
            debug!("Name is marked as present. Writing.");
            w.write_all(&stringdata::to_data(self.name_string.as_ref().unwrap()))?;
        }

        if *self.header().link_flags() & LinkFlags::HAS_RELATIVE_PATH
            == LinkFlags::HAS_RELATIVE_PATH {

            if self.relative_path == None { error!("Relative path not specified but expected!") }
            debug!("Relative path is marked as present. Writing.");
            w.write_all(&stringdata::to_data(self.relative_path.as_ref().unwrap()))?;
        }

        if *self.header().link_flags() & LinkFlags::HAS_WORKING_DIR
            == LinkFlags::HAS_WORKING_DIR {

            if self.working_dir == None { error!("Working Directory not specified but expected!") }
            debug!("Working dir is marked as present. Writing.");
            w.write_all(&stringdata::to_data(self.working_dir.as_ref().unwrap()))?;
        }

        if *self.header().link_flags() & LinkFlags::HAS_ARGUMENTS
            == LinkFlags::HAS_ARGUMENTS {

            if self.icon_location == None { error!("Arguments not specified but expected!") }
            debug!("Arguments are marked as present. Writing.");
            w.write_all(&stringdata::to_data(self.command_line_arguments.as_ref().unwrap()))?;
        }

        if *self.header().link_flags() & LinkFlags::HAS_ICON_LOCATION
            == LinkFlags::HAS_ICON_LOCATION {

            if self.icon_location == None { error!("Icon Location not specified but expected!") }
            debug!("Icon Location is marked as present. Writing.");
            w.write_all(&stringdata::to_data(self.icon_location.as_ref().unwrap()))?;
        }

        Ok(())
    }

    /// Open and parse a shell link
    pub fn open<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Self> {
        debug!("Opening {:?}", path.as_ref());
        let mut r = BufReader::new(File::open(path)?);
        let mut data = vec![];
        trace!("Reading file.");
        r.read_to_end(&mut data)?;

        trace!("Parsing shell header.");
        let shell_link_header = header::ShellLinkHeader::from(&data[0..0x4c]);
        debug!("Shell header: {:#?}", shell_link_header);

        let mut cursor = 0x4c;

        let mut linktarget_id_list = None;
        if *shell_link_header.link_flags() & LinkFlags::HAS_LINK_TARGET_ID_LIST
            == LinkFlags::HAS_LINK_TARGET_ID_LIST {

            debug!("A LinkTargetIDList is marked as present. Parsing now.");
            debug!("Cursor position: 0x{:x}", cursor);
            let list = linktarget::LinkTargetIdList::from(&data[cursor..]);
            debug!("{:?}", list);
            cursor += list.size as usize + 2; // add LinkTargetSize size
            linktarget_id_list = Some(list);
        }

        let mut link_info = None;
        if *shell_link_header.link_flags() & LinkFlags::HAS_LINK_INFO
            == LinkFlags::HAS_LINK_INFO {

            debug!("LinkInfo is marked as present. Parsing now.");
            debug!("Cursor position: 0x{:x}", cursor);
            let info = linkinfo::LinkInfo::from(&data[cursor..]);
            debug!("{:?}", info);
            cursor += info.size as usize;
            link_info = Some(info);
        }

        let mut name_string = None;
        let mut relative_path = None;
        let mut working_dir = None;
        let mut command_line_arguments = None;
        let mut icon_location = None;

        if *shell_link_header.link_flags() & LinkFlags::HAS_NAME
            == LinkFlags::HAS_NAME {

            debug!("Name is marked as present. Parsing now.");
            debug!("Cursor position: 0x{:x}", cursor);
            let (len, data) = stringdata::parse_string(&data[cursor..]);
            name_string = Some(data);
            cursor += len as usize + 2; // add len bytes
        }

        if *shell_link_header.link_flags() & LinkFlags::HAS_RELATIVE_PATH
            == LinkFlags::HAS_RELATIVE_PATH {

            debug!("Relative path is marked as present. Parsing now.");
            debug!("Cursor position: 0x{:x}", cursor);
            let (len, data) = stringdata::parse_string(&data[cursor..]);
            relative_path = Some(data);
            cursor += len as usize + 2; // add len bytes
        }

        if *shell_link_header.link_flags() & LinkFlags::HAS_WORKING_DIR
            == LinkFlags::HAS_WORKING_DIR {

            debug!("Working dir is marked as present. Parsing now.");
            debug!("Cursor position: 0x{:x}", cursor);
            let (len, data) = stringdata::parse_string(&data[cursor..]);
            working_dir = Some(data);
            cursor += len as usize + 2; // add len bytes
        }

        if *shell_link_header.link_flags() & LinkFlags::HAS_ARGUMENTS
            == LinkFlags::HAS_ARGUMENTS {

            debug!("Arguments are marked as present. Parsing now.");
            debug!("Cursor position: 0x{:x}", cursor);
            let (len, data) = stringdata::parse_string(&data[cursor..]);
            command_line_arguments = Some(data);
            cursor += len as usize + 2; // add len bytes
        }

        if *shell_link_header.link_flags() & LinkFlags::HAS_ICON_LOCATION
            == LinkFlags::HAS_ICON_LOCATION {

            debug!("Icon Location is marked as present. Parsing now.");
            debug!("Cursor position: 0x{:x}", cursor);
            let (len, data) = stringdata::parse_string(&data[cursor..]);
            icon_location = Some(data);
            cursor += len as usize + 2; // add len bytes
        }

        let mut extra_data = Vec::new();

        loop {
            if data[cursor..].len() < 4 {
                warn!("The ExtraData length is invalid.");
                break; // Probably an error?
            }
            debug!("Parsing ExtraData");
            debug!("Cursor position: 0x{:x}", cursor);
            let query = LE::read_u32(&data[cursor..]);
            if query < 0x04 {
                break;
            }
            extra_data.push(extradata::ExtraData::from(&data[cursor..]));
            cursor += query as usize;
        }

        let _remaining_data = &data[cursor..];

        Ok(Self {
            shell_link_header,
            linktarget_id_list,
            link_info,
            name_string,
            relative_path,
            working_dir,
            command_line_arguments,
            icon_location,
            extra_data,
        })
    }

    /// Get the header of the shell link
    pub fn header(&self) -> &ShellLinkHeader {
        &self.shell_link_header
    }

    /// Get a mutable instance of the shell link's header
    pub fn header_mut(&mut self) -> &mut ShellLinkHeader {
        &mut self.shell_link_header
    }

    /// Get the shell link's name, if set
    pub fn name(&self) -> &Option<String> {
        &self.name_string
    }

    /// Set the shell link's name
    pub fn set_name(&mut self, name: Option<String>) {
        let lf = *self.header().link_flags();
        if let Some(_) = name {
            self.header_mut().set_link_flags(lf | LinkFlags::HAS_NAME);
        } else {
            if lf & LinkFlags::HAS_NAME == LinkFlags::HAS_NAME {
                self.header_mut().set_link_flags(lf - LinkFlags::HAS_NAME);
            }
        }
        self.name_string = name;
    }

    /// Get the shell link's relative path, if set
    pub fn relative_path(&self) -> &Option<String> {
        &self.relative_path
    }

    /// Set the shell link's relative path
    pub fn set_relative_path(&mut self, relative_path: Option<String>) {
        let lf = *self.header().link_flags();
        if let Some(_) = relative_path {
            self.header_mut().set_link_flags(lf | LinkFlags::HAS_RELATIVE_PATH);
        } else {
            if lf & LinkFlags::HAS_RELATIVE_PATH == LinkFlags::HAS_RELATIVE_PATH {
                self.header_mut().set_link_flags(lf - LinkFlags::HAS_RELATIVE_PATH);
            }
        }
        self.relative_path = relative_path;
    }

    /// Get the shell link's working directory, if set
    pub fn working_dir(&self) -> &Option<String> {
        &self.working_dir
    }

    /// Set the shell link's working directory
    pub fn set_working_dir(&mut self, working_dir: Option<String>) {
        let lf = *self.header().link_flags();
        if let Some(_) = working_dir {
            self.header_mut().set_link_flags(lf | LinkFlags::HAS_WORKING_DIR);
        } else {
            if lf & LinkFlags::HAS_WORKING_DIR == LinkFlags::HAS_WORKING_DIR {
                self.header_mut().set_link_flags(lf - LinkFlags::HAS_WORKING_DIR);
            }
        }
        self.working_dir = working_dir;
    }

    /// Get the shell link's arguments, if set
    pub fn arguments(&self) -> &Option<String> {
        &self.command_line_arguments
    }

    /// Set the shell link's arguments
    pub fn set_arguments(&mut self, arguments: Option<String>) {
        let lf = *self.header().link_flags();
        if let Some(_) = arguments {
            self.header_mut().set_link_flags(lf | LinkFlags::HAS_ARGUMENTS);
        } else {
            if lf & LinkFlags::HAS_ARGUMENTS == LinkFlags::HAS_ARGUMENTS {
                self.header_mut().set_link_flags(lf - LinkFlags::HAS_ARGUMENTS);
            }
        }
        self.command_line_arguments = arguments;
    }

    /// Get the shell link's icon location, if set
    pub fn icon_location(&self) -> &Option<String> {
        &self.icon_location
    }

    /// Set the shell link's icon location
    pub fn set_icon_location(&mut self, icon_location: Option<String>) {
        let lf = *self.header().link_flags();
        if let Some(_) = icon_location {
            self.header_mut().set_link_flags(lf | LinkFlags::HAS_ICON_LOCATION);
        } else {
            if lf & LinkFlags::HAS_ICON_LOCATION == LinkFlags::HAS_ICON_LOCATION {
                self.header_mut().set_link_flags(lf - LinkFlags::HAS_ICON_LOCATION);
            }
        }
        self.icon_location = icon_location;
    }
}
