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
#[allow(unused)]
use log::{trace, debug, info, warn, error};

use std::io::{prelude::*, BufReader, BufWriter};
use std::fs::File;

mod header;
pub use header::{ShellLinkHeader, LinkFlags, FileAttributeFlags, HotKeyFlags, HotKeyFlagsLowByte, HotKeyFlagsHighByte, ShowCommand};

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
    pub shell_link_header: header::ShellLinkHeader,
    pub linktarget_id_list: Option<linktarget::LinkTargetIdList>,
    pub link_info: Option<linkinfo::LinkInfo>,
    pub name_string: Option<String>,
    pub relative_path: Option<String>,
    pub working_dir: Option<String>,
    pub command_line_arguments: Option<String>,
    pub icon_location: Option<String>,
    pub extra_data: Vec<extradata::ExtraData>,
}

impl ShellLink {

    /// Create a new ShellLink pointing to a location, with otherwise default settings.
    pub fn new_simple<S: Into<String>>(_to: S) -> Self {
        unimplemented!()
    }

    /// Create a new ShellLink, left fairly blank for your own customisation.
    pub fn new() -> Self {
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

    /// Save a shell link
    pub fn save<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<()> {
        let mut w = BufWriter::new(File::create(path)?);

        let header_data: [u8; 0x4c] = self.shell_link_header.into();
        w.write_all(&header_data)?;

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
        if shell_link_header.link_flags & LinkFlags::HAS_LINK_TARGET_ID_LIST
            == LinkFlags::HAS_LINK_TARGET_ID_LIST {

            debug!("A LinkTargetIDList is marked as present. Parsing now.");
            let list = linktarget::LinkTargetIdList::from(&data[cursor..]);
            debug!("{:?}", list);
            cursor += list.size as usize;
            linktarget_id_list = Some(list);
        }

        let mut link_info = None;
        if shell_link_header.link_flags & LinkFlags::HAS_LINK_INFO
            == LinkFlags::HAS_LINK_INFO {

            debug!("LinkInfo is marked as present. Parsing now.");
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

        /*loop {
            if data.len() < 4 {
                break; // Probably an error?
            }
            let query = LE::read_u32(&data[cursor..]);
            if query < 0x04 {
                break;
            }
            extra_data.push(extradata::ExtraData::from(&data[cursor..]));
            cursor += query as usize;
        }*/

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

fn read_enum_u16(data: &[u8]) -> u16 {
    assert!(data.len() >= 2);

    ((data[0] as u16) << 8) +
     (data[1] as u16)
}

fn read_enum_u32(data: &[u8]) -> u32 {
    assert!(data.len() >= 4);

    ((data[0] as u32) << 24) +
    ((data[1] as u32) << 16) +
    ((data[2] as u32) << 8) +
     (data[3] as u32)
}

#[cfg(test)]
mod tests {
    #[test]
    fn read_enum_test() {
        let data: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78, 0x9a];
        assert_eq!(super::read_enum_u16(&data[0..]), 0x1234);
        assert_eq!(super::read_enum_u32(&data[0..]), 0x12345678);
    }
}
