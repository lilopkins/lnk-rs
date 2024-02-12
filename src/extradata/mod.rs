use binread::{BinRead, BinReaderExt};
#[allow(unused)]
use log::{debug, error, info, trace, warn};

#[cfg(feature="serde")]
use serde::Serialize;

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
#[derive(Clone, Debug, BinRead)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[br(import(_block_size: u32))]
pub enum ExtraDataBlock {
    #[br(magic = 0xa0000002u32)]
    ConsoleProps(#[br(args(_block_size,))] ConsoleDataBlock),
    #[br(magic = 0xa0000004u32)]
    ConsoleFeProps(#[br(args(_block_size,))] ConsoleFEDataBlock),
    #[br(magic = 0xa0000006u32)]
    DarwinProps(#[br(args(_block_size,))] DarwinDataBlock),
    #[br(magic = 0xa0000001u32)]
    EnvironmentProps(#[br(args(_block_size,))] EnvironmentVariableDataBlock),
    #[br(magic = 0xa0000007u32)]
    IconEnvironmentProps(#[br(args(_block_size,))] IconEnvironmentDataBlock),
    #[br(magic = 0xa000000bu32)]
    KnownFolderProps(#[br(args(_block_size,))] KnownFolderDataBlock),
    #[br(magic = 0xa0000009u32)]
    PropertyStoreProps(#[br(args(_block_size,))] PropertyStoreDataBlock),
    #[br(magic = 0xa0000008u32)]
    ShimProps(#[br(args(_block_size,))] ShimDataBlock),
    #[br(magic = 0xa0000005u32)]
    SpecialFolderProps(#[br(args(_block_size,))] SpecialFolderDataBlock),
    #[br(magic = 0xa0000003u32)]
    TrackerProps(#[br(args(_block_size,))] TrackerDataBlock),
    #[br(magic = 0xa000000au32)]
    VistaAndAboveIdListProps(#[br(args(_block_size,))] VistaAndAboveIdListDataBlock),
}

#[derive(Default, Debug)]
#[allow(missing_docs, unused)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ExtraData {
    blocks: Vec<ExtraDataBlock>,
}

impl BinRead for ExtraData {
    type Args = ();

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        _options: &binread::ReadOptions,
        _args: Self::Args,
    ) -> binread::prelude::BinResult<Self> {
        let mut blocks = Vec::new();
        loop {
            let block_size: u32 = reader.read_le()?;
            if block_size == 0 {
                break;
            } else {
                let block: ExtraDataBlock = reader.read_le_args((block_size,))?;
                blocks.push(block);
            }
        }
        Ok(Self { blocks })
    }
}
