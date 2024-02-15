use binread::BinRead;
use byteorder::{ByteOrder, LE};
#[allow(unused)]
use log::{debug, error, info, trace, warn};

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{idlist::IdList, itemid::ItemID};

/// The LinkTargetIDList structure specifies the target of the link. The presence of this optional
/// structure is specified by the HasLinkTargetIDList bit (LinkFlagssection 2.1.1) in the
/// ShellLinkHeader(section2.1).
#[derive(Clone, Debug, Default, BinRead)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct LinkTargetIdList {
    /// The size, in bytes, of the IDList field.
    pub size: u16,
    /// A stored IDList structure (section 2.2.1), which contains the item ID list. An IDList
    /// structure conforms to the following ABNF \[RFC5234\]:
    ///   `IDLIST = *ITEMID TERMINALID`
    #[br(args(size))]
    id_list: IdList,
}

impl LinkTargetIdList {
    /// returns a reference to internal list of [`ItemID`] items
    pub fn id_list(&self) -> &Vec<ItemID> {
        self.id_list.item_id_list()
    }
}

impl From<LinkTargetIdList> for Vec<u8> {
    fn from(val: LinkTargetIdList) -> Self {
        let mut data = Vec::new();

        let size = 2u16;
        LE::write_u16(&mut data[0..2], size);
        for id in val.id_list() {
            let mut other_data = Into::<Vec<u8>>::into(id.clone());
            data.append(&mut other_data);
        }

        data
    }
}
