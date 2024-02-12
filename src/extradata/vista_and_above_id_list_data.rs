use std::mem::size_of;

use binread::BinRead;
use getset::Getters;

#[cfg(feature="serde")]
use serde::Serialize;

use crate::linktarget::IdList;

/// The VistaAndAboveIDListDataBlock structure specifies an alternate
/// IDList that can be used instead of the LinkTargetIDList structure
/// (section 2.2) on platforms that support it.
#[derive(Clone, Debug, BinRead, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[br(import(block_size: u32), pre_assert(block_size >= 0x0000_0000A))]
#[get(get="pub")]
#[allow(unused)]
pub struct VistaAndAboveIdListDataBlock {
    /// An IDList structure (section 2.2.1).
    #[br(args(u16::try_from(block_size).unwrap() - u16::try_from(2*size_of::<u32>()).unwrap()))]
    id_list: IdList,
}
