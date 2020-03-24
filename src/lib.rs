#![warn(missing_docs)]

//! # Shell Link parser and writer for Rust.
//! Works on any OS - although only really useful in Windows, this library can parse and write
//! .lnk files, a shell link, that can be understood by Windows.
//!
//! The full specification of these files can be found at
//! [Microsoft's Website](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-shllink/16cb4ca1-9339-4d0c-a68d-bf1d6cc0f943).
//!
//! ## Example
//! A simple example appears as follows:
//! ```rust
//! use lnk::ShellLink;
//! // ...
//! ShellLink::new_simple(r"C:\Windows\System32\notepad.exe");
//! ```

use byteorder::{ByteOrder, LE};

use std::io::{prelude::*, BufReader, BufWriter};
use std::fs::File;

mod header;
pub use header::{LinkFlags, FileAttributeFlags, HotKeyFlags, HotKeyFlagsLowByte, HotKeyFlagsHighByte};

mod linktarget;
pub use linktarget::{};

mod linkinfo;
pub use linkinfo::{};

mod stringdata;
pub use stringdata::{};

mod extradata;
pub use extradata::{};

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

impl ShellLink {

    /// Create a new ShellLink pointing to a location, with otherwise default settings.
    pub fn new_simple<S: Into<String>>(_to: S) -> Self {
        unimplemented!()
    }

    /// Create a new ShellLink, left fairly blank for your own customisation.
    pub fn new() -> Self {
        Self {
            shell_link_header: header::ShellLinkHeader::new(),
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

    /// Save a shell link
    pub fn save<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<()> {
        let mut w = BufWriter::new(File::create(path)?);

        let header_data = self.shell_link_header.to_data();
        w.write_all(&header_data)?;

        Ok(())
    }

    /// Open and parse a shell link
    pub fn open<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Self> {
        let mut r = BufReader::new(File::open(path)?);
        let mut data = vec![];
        r.read_to_end(&mut data)?;

        let mut shell_link_header = header::ShellLinkHeader::new();
        shell_link_header.from_data(&data[0..0x4c]);

        let mut cursor = 0x4c;

        let mut linktarget_id_list = None;
        if shell_link_header.link_flags & LinkFlags::HAS_LINK_TARGET_ID_LIST
            == LinkFlags::HAS_LINK_TARGET_ID_LIST {

            let list = linktarget::LinkTargetIdList::from(&data[cursor..]);
            cursor += list.size as usize;
            linktarget_id_list = Some(list);
        }

        let mut link_info = None;
        if shell_link_header.link_flags & LinkFlags::HAS_LINK_INFO
            == LinkFlags::HAS_LINK_INFO {

            let info = linkinfo::LinkInfo::from(&data[cursor..]);
            cursor += info.size as usize;
            link_info = Some(info);
        }

        let mut name_string = None;
        let mut relative_path = None;
        let mut working_dir = None;
        let mut command_line_arguments = None;
        let mut icon_location = None;

        if shell_link_header.link_flags & LinkFlags::HAS_NAME
            == LinkFlags::HAS_NAME {

            let (len, data) = stringdata::parse_string(&data[cursor..]);
            name_string = Some(data);
            cursor += len as usize;
        }

        if shell_link_header.link_flags & LinkFlags::HAS_RELATIVE_PATH
            == LinkFlags::HAS_RELATIVE_PATH {

            let (len, data) = stringdata::parse_string(&data[cursor..]);
            relative_path = Some(data);
            cursor += len as usize;
        }

        if shell_link_header.link_flags & LinkFlags::HAS_WORKING_DIR
            == LinkFlags::HAS_WORKING_DIR {

            let (len, data) = stringdata::parse_string(&data[cursor..]);
            working_dir = Some(data);
            cursor += len as usize;
        }

        if shell_link_header.link_flags & LinkFlags::HAS_ARGUMENTS
            == LinkFlags::HAS_ARGUMENTS {

            let (len, data) = stringdata::parse_string(&data[cursor..]);
            command_line_arguments = Some(data);
            cursor += len as usize;
        }

        if shell_link_header.link_flags & LinkFlags::HAS_ICON_LOCATION
            == LinkFlags::HAS_ICON_LOCATION {

            let (len, data) = stringdata::parse_string(&data[cursor..]);
            icon_location = Some(data);
            cursor += len as usize;
        }

        let mut extra_data = Vec::new();

        loop {
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
}
