use byteorder::{ByteOrder, LE};
use log::debug;
use packed_struct::PackedStructSlice;

use crate::linktarget::ItemID;

/// The VistaAndAboveIDListDataBlock structure specifies an alternate
/// IDList that can be used instead of the LinkTargetIDList structure
/// (section 2.2) on platforms that support it.
#[derive(Clone, Debug)]
pub struct VistaAndAboveIdListDataBlock {
    /// An IDList structure (section 2.2.1).
    pub id_list: Vec<ItemID>,
}

impl PackedStructSlice for VistaAndAboveIdListDataBlock {
    fn packed_bytes_size(_opt_self: Option<&Self>) -> packed_struct::PackingResult<usize> {
        unimplemented!()
    }

    fn pack_to_slice(&self, _output: &mut [u8]) -> packed_struct::PackingResult<()> {
        unimplemented!()
    }

    fn unpack_from_slice(src: &[u8]) -> packed_struct::PackingResult<Self> {
        let mut id_list = Vec::new();
        let mut inner_data = &src[8..];
        let mut offset = 0;
        loop {
            // Check for terminator
            if LE::read_u16(&src[offset..]) == 0 {
                break;
            }

            // Read an ItemID
            let id = ItemID::unpack_from_slice(inner_data)?;
            debug!("Read {:?}", id);
            let size = id.size;
            id_list.push(id);
            inner_data = &inner_data[(size as usize)..];
            offset += size as usize;
        }
        Ok(Self { id_list })
    }
}
