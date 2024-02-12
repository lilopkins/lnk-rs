use std::fmt;

use binread::{BinRead, BinReaderExt};
use byteorder::{ByteOrder, LE};
use getset::Getters;
#[allow(unused)]
use log::{debug, error, info, trace, warn};

#[cfg(feature="serde")]
use serde::Serialize;

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
        &self.id_list.item_id_list
    }
}

/// The stored IDList structure specifies the format of a persisted item ID list.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct IdList {
    item_id_list: Vec<ItemID>,
}

impl BinRead for IdList {
    type Args = (u16,);

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        _options: &binread::ReadOptions,
        args: Self::Args,
    ) -> binread::prelude::BinResult<Self> {
        let mut item_id_list = Vec::new();
        let mut bytes_to_read = args.0;
        trace!("ID List size: {bytes_to_read}");
        while bytes_to_read > 0 {

            // an IDList contains any number of ItemID structures,
            // followed by TerminalID which has a size of 2 bytes.
            // So, if there are less than 2 bytes available, there
            // is something wrong
            if bytes_to_read < 2 {
                return Err(binread::error::Error::AssertFail{
                    pos: reader.stream_position()?,
                    message: "not enough bytes to read".to_string(),
                });
            }

            let item_id: ItemID = reader.read_le()?;
            
            // if the item has a size of zero, then this
            // is the terminator
            if *item_id.size() == 0 {
                assert_eq!(bytes_to_read, 2);
                break;
            }
            
            bytes_to_read -= item_id.size();
            item_id_list.push(item_id);
        }

        Ok(Self{item_id_list})
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

/// The stored IDList structure specifies the format of a persisted item ID list.
#[derive(Clone, BinRead, Default, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[getset(get="pub")]
pub struct ItemID {
    /// A 16-bit, unsigned integer that specifies the size, in bytes, of the ItemID structure,
    /// including the ItemIDSize field.
    #[br(assert(size == 0 || size>2))]
    pub(crate) size: u16,

    /// The shell data source-defined data that specifies an item.
    #[br(if(size > 0), count=if size > 0 {size - 2} else {0})]
    data: Vec<u8>,
}

impl fmt::Debug for ItemID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ItemID (raw data size {})", self.size)
    }
}

impl From<&[u8]> for ItemID {
    fn from(data: &[u8]) -> Self {
        let mut id = Self::default();

        id.size = LE::read_u16(data);
        id.data = Vec::from(&data[2..(id.size as usize)]);

        id
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
