
use encoding_rs::Encoding;

use crate::LinkFlags;

/// enum to select which string encoding should be used
#[derive(Copy, Clone, Debug)]
pub enum StringEncoding {
    /// use the system default code page
    CodePage(&'static Encoding),

    /// use UNICODE (which is UTF-16LE on Windows)
    Unicode,
}

impl StringEncoding {
    /// creates string encoding information based on the given [`LinkFlags`]
    /// and the default encoding
    pub fn from(link_flags: LinkFlags, default_codepage: &'static Encoding) -> Self {
        if link_flags & LinkFlags::IS_UNICODE == LinkFlags::IS_UNICODE {
            Self::Unicode
        } else {
            Self::CodePage(default_codepage)
        }
    }
}
