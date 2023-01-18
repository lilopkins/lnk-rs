use std::fmt;

use byteorder::{ByteOrder, LE};
#[allow(unused)]
use log::{debug, error, info, trace, warn};
use packed_struct::prelude::*;

/// The LinkTargetIDList structure specifies the target of the link. The presence of this optional
/// structure is specified by the HasLinkTargetIDList bit (LinkFlagssection 2.1.1) in the
/// ShellLinkHeader(section2.1).
#[derive(Clone, Debug, Default)]
pub struct LinkTargetIdList {
    /// The size, in bytes, of the IDList field.
    pub size: u16,
    /// A stored IDList structure (section 2.2.1), which contains the item ID list. An IDList
    /// structure conforms to the following ABNF \[RFC5234\]:
    ///   `IDLIST = *ITEMID TERMINALID`
    pub id_list: Vec<ItemID>,
}

impl LinkTargetIdList {
    /// Safely add an `ItemID` to this `LinkTargetIdList`, updating the size field.
    pub fn add(&mut self, item_id: ItemID) {
        self.size += item_id.size;
        self.id_list.push(item_id);
    }
}

impl PackedStructSlice for LinkTargetIdList {
    fn packed_bytes_size(opt_self: Option<&Self>) -> packed_struct::PackingResult<usize> {
        Ok(opt_self.unwrap_or(&Default::default()).size as usize + 2)
    }

    fn pack_to_slice(&self, output: &mut [u8]) -> packed_struct::PackingResult<()> {
        LE::write_u16(&mut output[0..2], self.size);
        let mut offset = 2;
        for id in &self.id_list {
            id.pack_to_slice(&mut output[offset..])?;
            offset += id.size as usize;
        }

        if offset != self.size as usize {
            return Err(PackingError::InvalidValue);
        }

        Ok(())
    }

    fn unpack_from_slice(src: &[u8]) -> packed_struct::PackingResult<Self> {
        let mut id_list = Self::default();
        id_list.size = LE::read_u16(&src[0..]);
        trace!("ID List size: {}", id_list.size);
        let mut inner_data = &src[2..(id_list.size as usize)];
        assert!(inner_data.len() == id_list.size as usize - 2);
        let mut read_bytes = 2;
        while read_bytes < id_list.size {
            // Read an ItemID
            let id = ItemID::unpack_from_slice(inner_data)?;
            debug!("Read {:?}", id);
            let size = id.size;
            id_list.id_list.push(id);
            inner_data = &inner_data[(size as usize)..];
            read_bytes += size;
        }
        Ok(id_list)
    }
}

/// The stored IDList structure specifies the format of a persisted item ID list.
#[derive(Clone, Default)]
pub struct ItemID {
    /// A 16-bit, unsigned integer that specifies the size, in bytes, of the ItemID structure,
    /// including the ItemIDSize field.
    pub size: u16,
    /// The shell data source-defined data that specifies an item.
    pub data: Vec<u8>,
}

impl ItemID {
    /// Safely create a new ItemID, correctly setting the `size` field.
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            size: data.len() as u16 + 2,
            data,
        }
    }

    /// Safely update the internal `data`, correctly setting the `size` field.
    pub fn update(&mut self, data: Vec<u8>) {
        self.size = data.len() as u16 + 2;
        self.data = data;
    }
}

impl fmt::Debug for ItemID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ItemID (raw data size {})", self.size)
    }
}

impl PackedStructSlice for ItemID {
    fn packed_bytes_size(opt_self: Option<&Self>) -> packed_struct::PackingResult<usize> {
        Ok(opt_self.unwrap_or(&Default::default()).size as usize)
    }

    fn pack_to_slice(&self, output: &mut [u8]) -> packed_struct::PackingResult<()> {
        if self.data.len() as u16 + 2 != self.size {
            return Err(PackingError::InvalidValue);
        }

        LE::write_u16(output, self.size);
        let mut offset = 2;
        for byte in &self.data {
            output[offset] = *byte;
            offset += 1;
        }
        Ok(())
    }

    fn unpack_from_slice(src: &[u8]) -> packed_struct::PackingResult<Self> {
        let mut id = Self::default();

        id.size = LE::read_u16(src);
        id.data = Vec::from(&src[2..(id.size as usize)]);

        Ok(id)
    }
}
