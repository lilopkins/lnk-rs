use encoding_rs::UTF_16LE;

use crate::LinkFlags;

/// enum to select which string encoding should be used
#[derive(Copy, Clone, Debug)]
pub enum StringEncoding {
    /// use the system default code page
    CodePage(crate::strings::Encoding),

    /// use UNICODE (which is UTF-16LE on Windows)
    Unicode,
}

impl StringEncoding {
    /// creates string encoding information based on the given [`LinkFlags`]
    /// and the default encoding
    pub fn from(link_flags: LinkFlags, default_codepage: crate::strings::Encoding) -> Self {
        if link_flags.contains(LinkFlags::IS_UNICODE) {
            Self::Unicode
        } else {
            Self::CodePage(default_codepage)
        }
    }

    /// returns the effective encoding
    pub fn encoding(&self) -> crate::strings::Encoding {
        match self {
            StringEncoding::CodePage(cp) => cp,
            StringEncoding::Unicode => UTF_16LE,
        }
    }
}
