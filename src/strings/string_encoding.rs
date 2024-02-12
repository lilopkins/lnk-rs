
use crate::LinkFlags;

/// enum to select which string encoding should be used
#[derive(Copy, Clone, Debug)]
pub enum StringEncoding {
    /// use the system default code page
    CodePage,

    /// use UNICODE (which is UTF-16LE on Windows)
    Unicode,
}

impl From<LinkFlags> for StringEncoding {
    fn from(link_flags: LinkFlags) -> Self {
        if link_flags & LinkFlags::IS_UNICODE == LinkFlags::IS_UNICODE {
            Self::Unicode
        } else {
            Self::CodePage
        }
    }
}
