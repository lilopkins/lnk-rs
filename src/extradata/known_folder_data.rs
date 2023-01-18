use byteorder::{ByteOrder, LE};
use packed_struct::prelude::PackedStruct;

/// The KnownFolderDataBlock structure specifies the location of a
/// known folder. This data can be used when a link target is a
/// known folder to keep track of the folder so that the link target
/// IDList can be translated when the link is loaded.
#[derive(Clone, Copy, Debug)]
pub struct KnownFolderDataBlock {
    /// A value in GUID packet representation ([MS-DTYP] section
    /// 2.3.4.2) that specifies the folder GUID ID.
    pub known_folder_id: u128,
    /// A 32-bit, unsigned integer that specifies the location
    /// of the ItemID of the first child segment of the IDList specified
    /// by KnownFolderID. This value is the offset, in bytes, into the
    /// link target IDList.
    pub offset: u32,
}

impl PackedStruct for KnownFolderDataBlock {
    type ByteArray = [u8; 20];

    fn pack(&self) -> packed_struct::PackingResult<Self::ByteArray> {
        unimplemented!()
    }

    fn unpack(src: &Self::ByteArray) -> packed_struct::PackingResult<Self> {
        let known_folder_id = LE::read_u128(src);
        let offset = LE::read_u32(&src[16..]);
        Ok(Self {
            known_folder_id,
            offset,
        })
    }
}
