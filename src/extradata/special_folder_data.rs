use packed_struct::prelude::PackedStruct;

/// The SpecialFolderDataBlock structure specifies the location of a
/// special folder. This data can be used when a link target is a
/// special folder to keep track of the folder, so that the link target
/// IDList can be translated when the link is loaded.
#[derive(Clone, Copy, Debug, PackedStruct)]
#[packed_struct(endian = "lsb")]
pub struct SpecialFolderDataBlock {
    /// A 32-bit, unsigned integer that specifies the folder integer ID.
    pub special_folder_id: u32,
    /// A 32-bit, unsigned integer that specifies the location of the
    /// ItemID of the first child segment of the IDList specified by
    /// SpecialFolderID. This value is the offset, in bytes, into the
    /// link target IDList.
    pub offset: u32,
}
