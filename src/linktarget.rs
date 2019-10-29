use byteorder::{ByteOrder, LE};

/// The LinkTargetIDList structure specifies the target of the link. The presence of this optional
/// structure is specified by the HasLinkTargetIDList bit (LinkFlagssection 2.1.1) in the
/// ShellLinkHeader(section2.1).
#[derive(Clone, Debug)]
pub struct LinkTargetIdList {
    /// The size, in bytes, of the IDList field.
    size: u16,
    /// A stored IDList structure (section 2.2.1), which contains the item ID list. An IDList
    /// structure conforms to the following ABNF [RFC5234]:
    ///   `IDLIST = *ITEMID TERMINALID`
    id_list: Vec<ItemID>,
}

impl LinkTargetIdList {

    pub fn new() -> Self {
        Self {
            size: 0,
            id_list: vec![],
        }
    }

    /// Read data into this struct from a `[u8]`.
    pub fn from_data(&mut self, data: &[u8]) {
        self.size = LE::read_u16(&data[0..]);
        let mut inner_data = &data[2..(self.size as usize)];
        while LE::read_u16(inner_data) != 0 {
            // Read an ItemID
            let size = LE::read_u16(inner_data);
            self.id_list.push(ItemID {
                size: size,
                data: Vec::from(&inner_data[2..(size as usize)]),
            });
            inner_data = &inner_data[(size as usize)..];
        }
    }
}

/// The stored IDList structure specifies the format of a persisted item ID list.
#[derive(Clone, Debug)]
pub struct ItemID {
    /// A 16-bit, unsigned integer that specifies the size, in bytes, of the ItemID structure,
    /// including the ItemIDSize field.
    size: u16,
    /// The shell data source-defined data that specifies an item.
    data: Vec<u8>,
}
