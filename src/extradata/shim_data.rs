use std::mem::size_of;

use binread::BinRead;
use encoding_rs::UTF_16LE;
use getset::Getters;

#[cfg(feature="lnk2json")]
use serde::Serialize;

use crate::strings::FixedSizeString;

/// The ShimDataBlock structure specifies the name of a shim that can
/// be applied when activating a link target.
#[derive(Clone, Debug, BinRead, Getters)]
#[cfg_attr(feature = "lnk2json", derive(Serialize))]
#[br(import(block_size: u32), pre_assert(block_size >= 0x0000_00088))]
#[get(get = "pub")]
#[allow(unused)]
pub struct ShimDataBlock {
    /// A Unicode string that specifies the name of a shim layer to apply
    /// to a link target when it is being activated.
    #[br(args(usize::try_from(block_size).unwrap() - 2*size_of::<u32>(), UTF_16LE), map=|s:FixedSizeString| s.to_string())]
    layer_name: String,
}
