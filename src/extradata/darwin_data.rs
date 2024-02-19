use binread::BinRead;
use encoding_rs::{Encoding, UTF_16LE};
use getset::Getters;

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::strings::FixedSizeString;

/// The DarwinDataBlock structure specifies an application identifier
/// that can be used instead of a link target IDList to install an
/// application when a shell link is activated.
#[derive(Clone, Debug, BinRead, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[br(import(block_size: u32, default_codepage: &'static Encoding), pre_assert(block_size == 0x0000_00314))]
#[get(get = "pub")]
#[allow(unused)]
pub struct DarwinDataBlock {
    /// A NULL–terminated string, defined by the system default code
    /// page, which specifies an application identifier. This field
    /// SHOULD be ignored.
    #[br(args(260, default_codepage), map=|s:FixedSizeString| s.to_string())]
    darwin_data_ansi: String,

    /// An optional, NULL–terminated, Unicode string that specifies
    /// an application identifier.
    #[br(args(520, UTF_16LE), map=|s: FixedSizeString| if s.is_empty() {None} else {Some(s.to_string())})]
    darwin_data_unicode: Option<String>,
}
