use binread::{BinRead, BinReaderExt};
use getset::Getters;
use log::trace;
use serde::Serialize;

use crate::itemid::ItemID;


/// The stored IDList structure specifies the format of a persisted item ID list.
#[derive(Clone, Debug, Default, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[getset(get="pub")]
pub struct IdList {
    /// Contains a list of item identifiers.
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
            if bytes_to_read == 2 && *item_id.size() == 0 {
                break;
            }
            
            bytes_to_read -= item_id.size();
            item_id_list.push(item_id);
        }

        Ok(Self{item_id_list})
    }
}
