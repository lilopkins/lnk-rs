use bitflags::bitflags;

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::binread_flags::binread_flags;

bitflags! {
    /// The LinkFlags structure defines bits that specify which shell linkstructures are present in
    /// the file format after the ShellLinkHeaderstructure (section 2.1).
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    pub struct LinkFlags: u32 {
        /// The shell link is saved with an item ID list (IDList). If this bit is set, a
        /// LinkTargetIDList structure (section 2.2) MUST follow the ShellLinkHeader. If this bit
        /// is not set, this structure MUST NOT be present.
        const HAS_LINK_TARGET_ID_LIST           = 0b0000_0000_0000_0000_0000_0000_0000_0001;
        /// The shell link is saved with link information. If this bit is set, a LinkInfo structure
        /// (section 2.3) MUST be present. If this bit is not set, this structure MUST NOT be
        /// present.
        const HAS_LINK_INFO                     = 0b0000_0000_0000_0000_0000_0000_0000_0010;
        /// The shell link is saved with a name string. If this bit is set, a NAME_STRING
        /// StringData structure (section 2.4) MUST be present. If this bit is not set, this
        /// structure MUST NOT be present.
        const HAS_NAME                          = 0b0000_0000_0000_0000_0000_0000_0000_0100;
        /// The shell link is saved with a relative path string. If this bit is set, a
        /// RELATIVE_PATH StringData structure (section 2.4) MUST be present. If this bit is not
        /// set, this structure MUST NOT be present.
        const HAS_RELATIVE_PATH                 = 0b0000_0000_0000_0000_0000_0000_0000_1000;
        /// The shell link is saved with a relative path string. If this bit is set, a
        /// WORKING_DIR StringData structure (section 2.4) MUST be present. If this bit is not
        /// set, this structure MUST NOT be present.
        const HAS_WORKING_DIR                   = 0b0000_0000_0000_0000_0000_0000_0001_0000;
        /// The shell link is saved with a relative path string. If this bit is set, a
        /// COMMAND_LINE_ARGUMENTS StringData structure (section 2.4) MUST be present. If this bit
        /// is not set, this structure MUST NOT be present.
        const HAS_ARGUMENTS                     = 0b0000_0000_0000_0000_0000_0000_0010_0000;
        /// The shell link is saved with a relative path string. If this bit is set, a
        /// ICON_LOCATION StringData structure (section 2.4) MUST be present. If this bit is not
        /// set, this structure MUST NOT be present.
        const HAS_ICON_LOCATION                 = 0b0000_0000_0000_0000_0000_0000_0100_0000;
        /// The shell link contains Unicode encoded strings. This bit SHOULD be set. If this bit is
        /// set, the StringData section contains Unicode-encoded strings; otherwise, it contains
        /// strings that are encoded using the system default code page
        const IS_UNICODE                        = 0b0000_0000_0000_0000_0000_0000_1000_0000;
        /// The LinkInfo structure (section 2.3) is ignored.
        const FORCE_NO_LINK_INFO                = 0b0000_0000_0000_0000_0000_0001_0000_0000;
        /// The shell link is saved with an EnvironmentVariableDataBlock (section 2.5.4).
        const HAS_EXP_STRING                    = 0b0000_0000_0000_0000_0000_0010_0000_0000;
        /// The target is run in a separate virtual machine when launching a link target that is a
        /// 16-bit application.
        const RUN_IN_SEPARATE_PROCESS           = 0b0000_0000_0000_0000_0000_0100_0000_0000;
        /// A bit that is undefined and MUST be ignored.
        const UNUSED1                           = 0b0000_0000_0000_0000_0000_1000_0000_0000;
        /// The shell link is saved with a DarwinDataBlock(section2.5.3).
        const HAS_DARWIN_ID                     = 0b0000_0000_0000_0000_0001_0000_0000_0000;
        /// The application is run as a different user when the target of the shell link is
        /// activated.
        const RUN_AS_USER                       = 0b0000_0000_0000_0000_0010_0000_0000_0000;
        /// The shell link is saved with an IconEnvironmentDataBlock (section 2.5.5).
        const HAS_EXP_ICON                      = 0b0000_0000_0000_0000_0100_0000_0000_0000;
        /// The file system location is represented in the shell namespace when the path to an item
        /// is parsed into an IDList.
        const NO_PIDL_ALIAS                     = 0b0000_0000_0000_0000_1000_0000_0000_0000;
        /// A bit that is undefined and MUST be ignored.
        const UNUSED2                           = 0b0000_0000_0000_0001_0000_0000_0000_0000;
        /// The shell link is saved with a ShimDataBlock(section2.5.8)
        const RUN_WITH_SHIM_LAYER               = 0b0000_0000_0000_0010_0000_0000_0000_0000;
        /// The TrackerDataBlock(section2.5.10)is ignored.
        const FORCE_NO_LINK_TRACK               = 0b0000_0000_0000_0100_0000_0000_0000_0000;
        /// The shell link attempts to collect target properties and store them in the
        /// PropertyStoreDataBlock(section2.5.7) when the link target is set.
        const ENABLE_TARGET_METADATA            = 0b0000_0000_0000_1000_0000_0000_0000_0000;
        /// The EnvironmentVariableDataBlock is ignored.
        const DISABLE_LINK_PATH_TRACKING        = 0b0000_0000_0001_0000_0000_0000_0000_0000;
        /// The SpecialFolderDataBlock(section2.5.9)and the KnownFolderDataBlock(section2.5.6)are
        /// ignored when loading the shell link. If this bit is set, these extra data blocks SHOULD
        /// NOT be saved when saving the shell link.
        const DISABLE_KNOWN_FOLDER_TRACKING     = 0b0000_0000_0010_0000_0000_0000_0000_0000;
        /// If the linkhas a KnownFolderDataBlock(section2.5.6), the unaliased form of the known
        /// folder IDList SHOULD be used when translating the target IDList at the time that the
        /// link is loaded.
        const DISABLE_KNOWN_FOLDER_ALIAS        = 0b0000_0000_0100_0000_0000_0000_0000_0000;
        /// Creating a link that references another link is enabled. Otherwise, specifying a link
        /// as the target IDList SHOULD NOT be allowed.
        const ALLOW_LINK_TO_LINK                = 0b0000_0000_1000_0000_0000_0000_0000_0000;
        /// When saving a link for which the target IDList is under a known folder, either the
        /// unaliased form of that known folder or the target IDList SHOULD be used.
        const UNALIAS_ON_SAVE                   = 0b0000_0001_0000_0000_0000_0000_0000_0000;
        /// The target IDList SHOULD NOT be stored; instead, the path specified in the
        /// EnvironmentVariableDataBlock(section2.5.4) SHOULD be used to refer to the target.
        const PREFER_ENVIRONMENT_PATH           = 0b0000_0010_0000_0000_0000_0000_0000_0000;
        /// When the target is a UNC name that refers to a location on a local machine, the local
        /// path IDList in the PropertyStoreDataBlock(section2.5.7) SHOULD be stored, so it can be
        /// used when the link is loaded on the local machine.
        const KEEP_LOCAL_ID_LIST_FOR_UNC_TARGET = 0b0000_0100_0000_0000_0000_0000_0000_0000;
    }
}

binread_flags!(LinkFlags, u32);
