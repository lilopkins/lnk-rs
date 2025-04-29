mod fixed_size_string;
mod null_terminated_string;
mod sized_string;
mod string_encoding;

pub use fixed_size_string::*;
pub use null_terminated_string::*;
pub use sized_string::*;
pub use string_encoding::*;

/// this type wraps [`encoding_rs::Encoding`] so that depending crates do not
/// need to to depend on [`encoding_rs`]
pub type Encoding = &'static encoding_rs::Encoding;

macro_rules! support_encoding {
    ($encoding:ident) => {
        pub static $encoding: crate::strings::Encoding = encoding_rs::$encoding;
    };
}

/// this module reexports all statics from `encoding_rs`
pub mod encoding {
    #![allow(missing_docs)]
    support_encoding!(WINDOWS_1250);
    support_encoding!(WINDOWS_1251);
    support_encoding!(WINDOWS_1252);
    support_encoding!(WINDOWS_1253);
    support_encoding!(WINDOWS_1254);
    support_encoding!(WINDOWS_1255);
    support_encoding!(WINDOWS_1256);
    support_encoding!(WINDOWS_1257);
    support_encoding!(WINDOWS_1258);
    support_encoding!(WINDOWS_874);
    support_encoding!(UTF_16LE);
}
