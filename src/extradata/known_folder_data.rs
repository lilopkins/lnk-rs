use binread::BinRead;
use getset::Getters;

#[cfg(feature="lnk2json")]
use serde::Serialize;

use crate::Guid;

/// The KnownFolderDataBlock structure specifies the location of a
/// known folder. This data can be used when a link target is a
/// known folder to keep track of the folder so that the link target
/// IDList can be translated when the link is loaded.
#[derive(Clone, Copy, Debug, BinRead, Getters)]
#[cfg_attr(feature = "lnk2json", derive(Serialize))]
#[br(import(block_size: u32), pre_assert(block_size == 0x0000_0001C))]
#[get(get="pub")]
#[allow(unused)]
pub struct KnownFolderDataBlock {
    /// A value in GUID packet representation ([MS-DTYP] section
    /// 2.3.4.2) that specifies the folder GUID ID.
    known_folder_id: Guid,
    /// A 32-bit, unsigned integer that specifies the location
    /// of the ItemID of the first child segment of the IDList specified
    /// by KnownFolderID. This value is the offset, in bytes, into the
    /// link target IDList.
    offset: u32,
}
