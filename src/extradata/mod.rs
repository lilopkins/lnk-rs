use byteorder::{ByteOrder, LE};
#[allow(unused)]
use log::{debug, error, info, trace, warn};

use self::{
    console_data::ConsoleDataBlock, console_fe_data::ConsoleFEDataBlock,
    darwin_data::DarwinDataBlock, environment_variable_data::EnvironmentVariableDataBlock,
    icon_environment_data::IconEnvironmentDataBlock, known_folder_data::KnownFolderDataBlock,
    property_store_data::PropertyStoreDataBlock, shim_data::ShimDataBlock,
    special_folder_data::SpecialFolderDataBlock, tracker_data::TrackerDataBlock,
    vista_and_above_id_list_data::VistaAndAboveIdListDataBlock,
};

pub mod console_data;
pub mod console_fe_data;
pub mod darwin_data;
pub mod environment_variable_data;
pub mod icon_environment_data;
pub mod known_folder_data;
pub mod property_store_data;
pub mod shim_data;
pub mod special_folder_data;
pub mod tracker_data;
pub mod vista_and_above_id_list_data;

/// ExtraData refers to a set of structures that convey additional information
/// about a link target. These optional structures can be present in an extra
/// data section that is appended to the basic Shell Link Binary File Format.
///
/// At the moment, ExtraData can only be read, not written to shortcuts.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum ExtraData {
    ConsoleProps(ConsoleDataBlock),
    ConsoleFeProps(ConsoleFEDataBlock),
    DarwinProps(DarwinDataBlock),
    EnvironmentProps(EnvironmentVariableDataBlock),
    IconEnvironmentProps(IconEnvironmentDataBlock),
    KnownFolderProps(KnownFolderDataBlock),
    PropertyStoreProps(PropertyStoreDataBlock),
    ShimProps(ShimDataBlock),
    SpecialFolderProps(SpecialFolderDataBlock),
    TrackerProps(TrackerDataBlock),
    VistaAndAboveIdListProps(VistaAndAboveIdListDataBlock),
}

impl From<&[u8]> for ExtraData {
    fn from(data: &[u8]) -> Self {
        let size = LE::read_u32(data) as usize;
        let sig = LE::read_u32(&data[4..]);
        debug!("Signature {:x}", sig);
        let data = &data[8..size];

        match sig {
            0xa0000002 => Self::ConsoleProps(ConsoleDataBlock::from(data)),
            0xa0000004 => Self::ConsoleFeProps(ConsoleFEDataBlock::from(data)),
            0xa0000006 => Self::DarwinProps(DarwinDataBlock::from(data)),
            0xa0000001 => Self::EnvironmentProps(EnvironmentVariableDataBlock::from(data)),
            0xa0000007 => Self::IconEnvironmentProps(IconEnvironmentDataBlock::from(data)),
            0xa000000b => Self::KnownFolderProps(KnownFolderDataBlock::from(data)),
            0xa0000009 => Self::PropertyStoreProps(PropertyStoreDataBlock::from(data)),
            0xa0000008 => Self::ShimProps(ShimDataBlock::from(data)),
            0xa0000005 => Self::SpecialFolderProps(SpecialFolderDataBlock::from(data)),
            0xa0000003 => Self::TrackerProps(TrackerDataBlock::from(data)),
            0xa000000a => Self::VistaAndAboveIdListProps(VistaAndAboveIdListDataBlock::from(data)),
            _ => panic!("Invalid extra data type!"),
        }
    }
}
