use binread::BinRead;
use encoding_rs::{UTF_16LE, WINDOWS_1252};
use getset::Getters;

use crate::strings::FixedSizeString;

/// The DarwinDataBlock structure specifies an application identifier
/// that can be used instead of a link target IDList to install an
/// application when a shell link is activated.
#[derive(Clone, Debug, BinRead, Getters)]
#[get(get = "pub")]
#[allow(unused)]
pub struct DarwinDataBlock {
    /// A NULL–terminated string, defined by the system default code
    /// page, which specifies an application identifier. This field
    /// SHOULD be ignored.
    #[br(args(260, WINDOWS_1252), map=|s:FixedSizeString| s.to_string())]
    darwin_data_ansi: String,

    /// An optional, NULL–terminated, Unicode string that specifies
    /// an application identifier.
    #[br(args(520, UTF_16LE), map=|s: FixedSizeString| if s.is_empty() {None} else {Some(s.to_string())})]
    darwin_data_unicode: Option<String>,
}
