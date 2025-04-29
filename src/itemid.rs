use core::fmt;

use binrw::BinRead;
use getset::Getters;
use serde::Serialize;

/// The stored IDList structure specifies the format of a persisted item ID list.
#[derive(Clone, BinRead, Default, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[getset(get = "pub")]
pub struct ItemID {
    /// A 16-bit, unsigned integer that specifies the size, in bytes, of the ItemID structure,
    /// including the ItemIDSize field.
    #[br(assert(size == 0 || size>2))]
    #[cfg_attr(feature = "serde", serde(skip))]
    size: u16,

    /// The shell data source-defined data that specifies an item.
    #[br(if(size > 0), count=if size > 0 {size - 2} else {0})]
    data: Vec<u8>,
}

impl fmt::Debug for ItemID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ItemID (raw data size {})", self.size)
    }
}
