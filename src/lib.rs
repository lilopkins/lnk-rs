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
//! ShellLink::new_simple("C:\\Windows\\System32\\notepad.exe");
//! ```

use std::io::{prelude::*, BufReader, BufWriter};
use std::fs::File;

mod header;
pub use header::{LinkFlags, FileAttributeFlags, HotKeyFlags, HotKeyFlagsLowByte, HotKeyFlagsHighByte};

mod linktarget;
pub use linktarget::{};

#[derive(Clone, Copy, Debug)]
struct LinkInfo {

}

#[derive(Clone, Copy, Debug)]
struct StringData {

}

#[derive(Clone, Copy, Debug)]
struct ExtraData {

}

/// A shell link
#[derive(Clone, Debug)]
pub struct ShellLink {
    shell_link_header: header::ShellLinkHeader,
    linktarget_id_list: Option<linktarget::LinkTargetIdList>,
    link_info: Option<LinkInfo>,
    string_data: Option<StringData>,
    extra_data: Vec<ExtraData>,
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
            string_data: None,
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

        let mut linktarget_id_list = None;
        if shell_link_header.link_flags & LinkFlags::HAS_LINK_TARGET_ID_LIST
            == LinkFlags::HAS_LINK_TARGET_ID_LIST {

            let list = linktarget::LinkTargetIdList::from(&data[0x4c..]);
            linktarget_id_list = Some(list);
        }

        let mut link_info = None;
        if shell_link_header.link_flags & LinkFlags::HAS_LINK_INFO
            == LinkFlags::HAS_LINK_INFO {

            link_info = Some(LinkInfo {});
        }

        Ok(Self {
            shell_link_header,
            linktarget_id_list,
            link_info,
            string_data: None,
            extra_data: vec![],
        })
    }
}
