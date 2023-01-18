use byteorder::{ByteOrder, LE};
#[allow(unused)]
use log::{debug, error, info, trace, warn};
use packed_struct::PackedStructSlice;

use self::{
    console_data::ConsoleDataBlock, console_fe_data::ConsoleFEDataBlock,
    darwin_data::DarwinDataBlock, environment_variable_data::EnvironmentVariableDataBlock,
    icon_environment_data::IconEnvironmentDataBlock, known_folder_data::KnownFolderDataBlock,
    property_store_data::PropertyStoreDataBlock, shim_data::ShimDataBlock,
    special_folder_data::SpecialFolderDataBlock, tracker_data::TrackerDataBlock,
    vista_and_above_id_list_data::VistaAndAboveIdListDataBlock,
};

/// The ConsoleDataBlock structure specifies the display settings to use
/// when a link target specifies an application that is run in a console
/// window.
pub mod console_data;

/// The ConsoleFEDataBlock structure specifies the code page to use
/// for displaying text when a link target specifies an application
/// that is run in a console window.
pub mod console_fe_data;

/// The DarwinDataBlock structure specifies an application identifier
/// that can be used instead of a link target IDList to install an
/// application when a shell link is activated.
pub mod darwin_data;

/// The EnvironmentVariableDataBlock structure specifies a path to
/// environment variable information when the link target refers to
/// a location that has a corresponding environment variable.
pub mod environment_variable_data;

/// The IconEnvironmentDataBlock structure specifies the path to an
/// icon. The path is encoded using environment variables, which makes
/// it possible to find the icon across machines where the locations
/// vary but are expressed using environment variables.
pub mod icon_environment_data;

/// The KnownFolderDataBlock structure specifies the location of a
/// known folder. This data can be used when a link target is a
/// known folder to keep track of the folder so that the link target
/// IDList can be translated when the link is loaded.
pub mod known_folder_data;

/// A PropertyStoreDataBlock structure specifies a set of properties
/// that can be used by applications to store extra data in the
/// shell link.
pub mod property_store_data;

/// The ShimDataBlock structure specifies the name of a shim that can
/// be applied when activating a link target.
pub mod shim_data;

/// The SpecialFolderDataBlock structure specifies the location of a
/// special folder. This data can be used when a link target is a
/// special folder to keep track of the folder, so that the link target
/// IDList can be translated when the link is loaded.
pub mod special_folder_data;

/// The TrackerDataBlock structure specifies data that can be used to
/// resolve a link target if it is not found in its original location
/// when the link is resolved. This data is passed to the Link
/// Tracking service [MS-DLTW] to find the link target.
pub mod tracker_data;

/// The VistaAndAboveIDListDataBlock structure specifies an alternate
/// IDList that can be used instead of the LinkTargetIDList structure
/// (section 2.2) on platforms that support it.
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

impl PackedStructSlice for ExtraData {
    fn packed_bytes_size(_opt_self: Option<&Self>) -> packed_struct::PackingResult<usize> {
        unimplemented!()
    }

    fn pack_to_slice(&self, _output: &mut [u8]) -> packed_struct::PackingResult<()> {
        unimplemented!()
    }

    fn unpack_from_slice(src: &[u8]) -> packed_struct::PackingResult<Self> {
        let size = LE::read_u32(src) as usize;
        let sig = LE::read_u32(&src[4..]);
        debug!("Signature {:x}", sig);
        let data = &src[8..size];

        Ok(match sig {
            0xa0000002 => Self::ConsoleProps(ConsoleDataBlock::unpack_from_slice(data)?),
            0xa0000004 => Self::ConsoleFeProps(ConsoleFEDataBlock::unpack_from_slice(data)?),
            0xa0000006 => Self::DarwinProps(DarwinDataBlock::unpack_from_slice(data)?),
            0xa0000001 => Self::EnvironmentProps(EnvironmentVariableDataBlock::unpack_from_slice(data)?),
            0xa0000007 => Self::IconEnvironmentProps(IconEnvironmentDataBlock::unpack_from_slice(data)?),
            0xa000000b => Self::KnownFolderProps(KnownFolderDataBlock::unpack_from_slice(data)?),
            0xa0000009 => Self::PropertyStoreProps(PropertyStoreDataBlock::unpack_from_slice(data)?),
            0xa0000008 => Self::ShimProps(ShimDataBlock::unpack_from_slice(data)?),
            0xa0000005 => Self::SpecialFolderProps(SpecialFolderDataBlock::unpack_from_slice(data)?),
            0xa0000003 => Self::TrackerProps(TrackerDataBlock::unpack_from_slice(data)?),
            0xa000000a => Self::VistaAndAboveIdListProps(VistaAndAboveIdListDataBlock::unpack_from_slice(data)?),
            _ => panic!("Invalid extra data type!"),
        })
    }
}
