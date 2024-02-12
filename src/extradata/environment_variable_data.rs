use binread::BinRead;
use encoding_rs::{UTF_16LE, WINDOWS_1252};
use getset::Getters;

use crate::strings::FixedSizeString;

/// The EnvironmentVariableDataBlock structure specifies a path to
/// environment variable information when the link target refers to
/// a location that has a corresponding environment variable.
#[derive(Clone, Debug, BinRead, Getters)]
#[br(import(block_size: u32), pre_assert(block_size == 0x0000_0314))]
#[get(get = "pub")]
#[allow(unused)]
pub struct EnvironmentVariableDataBlock {
    /// A NULL-terminated string, defined by the system default code
    /// page, which specifies a path to environment variable information.
    #[br(args(260, WINDOWS_1252), map=|s:FixedSizeString| s.to_string())]
    target_ansi: String,
    /// An optional, NULL-terminated, Unicode string that specifies a path
    /// to environment variable information.
    #[br(args(520, UTF_16LE), map=|s: FixedSizeString| if s.is_empty() {None} else {Some(s.to_string())})]
    target_unicode: Option<String>,
}
