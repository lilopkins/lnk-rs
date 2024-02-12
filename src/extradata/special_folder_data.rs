use binread::BinRead;
use getset::Getters;

/// The SpecialFolderDataBlock structure specifies the location of a
/// special folder. This data can be used when a link target is a
/// special folder to keep track of the folder, so that the link target
/// IDList can be translated when the link is loaded.
#[derive(Clone, Copy, Debug, BinRead, Getters)]
#[br(import(block_size: u32), pre_assert(block_size == 0x0000_00010))]
#[get(get="pub")]
#[allow(unused)]
pub struct SpecialFolderDataBlock {
    /// A 32-bit, unsigned integer that specifies the folder integer ID.
    special_folder_id: u32,
    /// A 32-bit, unsigned integer that specifies the location of the
    /// ItemID of the first child segment of the IDList specified by
    /// SpecialFolderID. This value is the offset, in bytes, into the
    /// link target IDList.
    offset: u32,
}
