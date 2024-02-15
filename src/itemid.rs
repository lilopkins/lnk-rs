use core::fmt;

use binread::BinRead;
use byteorder::{ByteOrder, LE};
use getset::Getters;
use serde::Serialize;


/// The stored IDList structure specifies the format of a persisted item ID list.
#[derive(Clone, BinRead, Default, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[getset(get="pub")]
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

impl From<ItemID> for Vec<u8> {
    fn from(val: ItemID) -> Self {
        let mut data = Vec::new();

        assert_eq!(val.data.len() as u16 + 2, val.size);

        LE::write_u16(&mut data, val.size);
        let mut other_data = val.data.clone();
        data.append(&mut other_data);

        data
    }
}
