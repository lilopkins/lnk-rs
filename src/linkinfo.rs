use binread::BinRead;
use bitflags::bitflags;
use encoding_rs::Encoding;
use getset::Getters;
use num_derive::{FromPrimitive, ToPrimitive};

use crate::{
    binread_flags::binread_flags,
    strings::{NullTerminatedString, StringEncoding},
    CurrentOffset,
};

#[cfg(feature="serde")]
use serde::Serialize;

/// The LinkInfo structure specifies information necessary to resolve a
/// linktarget if it is not found in its original location. This includes
/// information about the volume that the target was stored on, the mapped
/// drive letter, and a Universal Naming Convention (UNC)form of the path
/// if one existed when the linkwas created. For more details about UNC
/// paths, see [MS-DFSNM] section 2.2.1.4
#[derive(Debug, BinRead, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[get(get="pub")]
#[allow(unused)]
#[br(import(default_codepage: &'static Encoding))]
pub struct LinkInfo {
    #[cfg_attr(feature = "serde", serde(skip))]
    start_offset: CurrentOffset,

    /// LinkInfoSize (4 bytes): A 32-bit, unsigned integer that specifies the
    /// size, in bytes, of the LinkInfo structure. All offsets specified in
    /// this structure MUST be less than this value, and all strings contained
    /// in this structure MUST fit within the extent defined by this size.
    link_info_size: u32,

    /// LinkInfoHeaderSize (4 bytes): A 32-bit, unsigned integer that
    /// specifies the size, in bytes, of the LinkInfo header section, which is
    /// composed of the LinkInfoSize, LinkInfoHeaderSize, LinkInfoFlags,
    /// VolumeIDOffset, LocalBasePathOffset, CommonNetworkRelativeLinkOffset,
    /// CommonPathSuffixOffset fields, and, if included, the
    /// LocalBasePathOffsetUnicode and CommonPathSuffixOffsetUnicode fields.
    link_info_header_size: u32,

    /// Flags that specify whether the VolumeID, LocalBasePath,
    /// LocalBasePathUnicode, and CommonNetworkRelativeLinkfields are present
    /// in this structure.
    link_info_flags: LinkInfoFlags,

    /// VolumeIDOffset (4 bytes): A 32-bit, unsigned integer that specifies the
    /// location of the VolumeID field. If the VolumeIDAndLocalBasePath flag is
    /// set, this value is an offset, in bytes, from the start of the LinkInfo
    /// structure; otherwise, this value MUST be zero.
    #[br(
        assert(
            if link_info_flags.has_volume_id_and_local_base_path() {
                volume_id_offset > 0 && volume_id_offset < link_info_size
            } else {
                volume_id_offset == 0
            }
        )
    )]
    volume_id_offset: u32,

    /// LocalBasePathOffset (4 bytes): A 32-bit, unsigned integer that
    /// specifies the location of the LocalBasePath field. If the
    /// VolumeIDAndLocalBasePath flag is set, this value is an offset, in
    /// bytes, from the start of the LinkInfo structure; otherwise, this value
    /// MUST be zero.
    #[br(
        assert(
            if link_info_flags.has_volume_id_and_local_base_path() {
                local_base_path_offset > 0 && local_base_path_offset < link_info_size
            } else {
                local_base_path_offset == 0
            }
        )
    )]
    local_base_path_offset: u32,

    /// CommonNetworkRelativeLinkOffset (4 bytes): A 32-bit, unsigned integer
    /// that specifies the location of the CommonNetworkRelativeLink field. If
    /// the CommonNetworkRelativeLinkAndPathSuffix flag is set, this value is
    /// an offset, in bytes, from the start of the LinkInfo structure;
    /// otherwise, this value MUST be zero.
    #[br(
        assert(
            if link_info_flags.has_common_network_relative_link_and_path_suffix() {
                common_network_relative_link_offset > 0 && common_network_relative_link_offset < link_info_size
            } else {
                common_network_relative_link_offset == 0
            }
        )
    )]
    common_network_relative_link_offset: u32,

    /// CommonPathSuffixOffset (4 bytes): A 32-bit, unsigned integer that
    /// specifies the location of the CommonPathSuffix field. This value is
    /// an offset, in bytes, from the start of the LinkInfo structure.
    #[br(assert(common_path_suffix_offset < link_info_size && common_path_suffix_offset != 0))]
    common_path_suffix_offset: u32,

    /// LocalBasePathOffsetUnicode (4 bytes): An optional, 32-bit, unsigned
    /// integer that specifies the location of the LocalBasePathUnicode field.
    /// If the VolumeIDAndLocalBasePath flag is set, this value is an offset,
    /// in bytes, from the start of the LinkInfo structure; otherwise, this
    /// value MUST be zero. This field can be present only if the value of the
    /// LinkInfoHeaderSize field is greater than or equal to 0x00000024.
    #[br(
        if(link_info_header_size >= 0x24),
        assert(
            if let Some(offset) = local_base_path_offset_unicode {
                if link_info_flags.has_volume_id_and_local_base_path(){
                    offset > 0 && offset < link_info_size
                } else {
                    false
                }
            } else {
                true
            }
        )
    )]
    local_base_path_offset_unicode: Option<u32>,

    /// CommonPathSuffixOffsetUnicode (4 bytes): An optional, 32-bit, unsigned
    /// integer that specifies the location of the CommonPathSuffixUnicode
    /// field. This value is an offset, in bytes, from the start of the
    /// LinkInfo structure. This field can be present only if the value of the
    /// LinkInfoHeaderSize field is greater than or equal to 0x00000024.
    #[br(
        if(link_info_header_size >= 0x24),
        assert (
            if let Some(offset) = common_path_suffix_offset_unicode {
                if link_info_flags.has_common_network_relative_link_and_path_suffix() {
                    offset > 0 && offset < link_info_size
                } else {
                    false
                }
            } else {true}
        )
    )]
    common_path_suffix_offset_unicode: Option<u32>,

    /// An optional VolumeID structure (section 2.3.1) that specifies
    /// information about the volume that the link target was on when the link
    /// was created. This field is present if the VolumeIDAndLocalBasePath
    /// flag is set.
    #[br(
        //seek_before(SeekFrom::Start((*start_offset.as_ref() + volume_id_offset).into())),
        if(link_info_flags.has_volume_id_and_local_base_path()),
        args(default_codepage)
    )]
    volume_id: Option<VolumeID>,

    /// An optional, NULL–terminated string, defined by the system default code
    /// page, which is used to construct the full path to the link item or link
    /// target by appending the string in the CommonPathSuffix field. This
    /// field is present if the VolumeIDAndLocalBasePath flag is set.
    #[br(
        if(link_info_flags.has_volume_id_and_local_base_path()),
        args(StringEncoding::CodePage(default_codepage)),
        map=|o: Option<NullTerminatedString>| o.map(|n| n.to_string())
    )]
    #[getset(skip)]
    local_base_path: Option<String>,

    /// An optional CommonNetworkRelativeLink structure (section 2.3.2) that
    /// specifies information about the network location where the link target
    /// is stored.
    #[br(
        if(link_info_flags.has_common_network_relative_link_and_path_suffix()),
        args(default_codepage)
    )]
    common_network_relative_link: Option<CommonNetworkRelativeLink>,
    
    /// A NULL–terminated string, defined by the system default code page,
    /// which is used to construct the full path to the link item or link
    /// target by being appended to the string in the LocalBasePath field.
    #[br(
        if(common_path_suffix_offset != 0),
        args(StringEncoding::CodePage(default_codepage)),
        map=|n: NullTerminatedString| n.to_string()
    )]
    #[getset(skip)]
    common_path_suffix: String,

    /// An optional, NULL–terminated, Unicode string that is used to construct
    /// the full path to the link item or link target by appending the string
    /// in the CommonPathSuffixUnicode field. This field can be present only
    /// if the VolumeIDAndLocalBasePath flag is set and the value of the
    /// LinkInfoHeaderSize field is greater than or equal to 0x00000024.
    #[br(
        if(link_info_header_size >= 0x24 && link_info_flags.has_volume_id_and_local_base_path()),
        args(StringEncoding::Unicode),
        map=|o: Option<NullTerminatedString>| o.map(|n| n.to_string())
    )]
    local_base_path_unicode: Option<String>,

    #[br(
        if(link_info_header_size >= 0x24 && common_path_suffix_offset_unicode.map(|o| o != 0).unwrap_or(false)),
        args(StringEncoding::Unicode),
        map=|o: Option<NullTerminatedString>| o.map(|n| n.to_string())
    )]
    common_path_suffix_unicode: Option<String>,
}

impl LinkInfo {
    /// An optional, NULL–terminated string, defined by the system default code
    /// page, which is used to construct the full path to the link item or link
    /// target by appending the string in the CommonPathSuffix field. This
    /// field is present if the VolumeIDAndLocalBasePath flag is set.
    pub fn local_base_path(&self) -> Option<&str> {
        self.local_base_path.as_ref().map(|x| x.as_ref())
    }
    /// A NULL–terminated string, defined by the system default code page,
    /// which is used to construct the full path to the link item or link
    /// target by being appended to the string in the LocalBasePath field.
    pub fn common_path_suffix(&self) -> &str {
        self.common_path_suffix.as_ref()
    }
}

impl From<LinkInfo> for Vec<u8> {
    fn from(_val: LinkInfo) -> Self {
        unimplemented!()
    }
}

bitflags! {
    /// Flags that specify whether the VolumeID, LocalBasePath, LocalBasePathUnicode,
    /// and CommonNetworkRelativeLink fields are present in this structure.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    pub struct LinkInfoFlags: u32 {
        /// If set, the VolumeIDand LocalBasePath fields are present, and their
        /// locations are specified by the values of the VolumeIDOffset and
        /// LocalBasePathOffset fields, respectively. If the value of the
        /// LinkInfoHeaderSize field is greater than or equal to 0x00000024, the
        /// LocalBasePathUnicode field is present, and its location is specified
        /// by the value of the LocalBasePathOffsetUnicode field. If not set,
        /// the VolumeID, LocalBasePath, and LocalBasePathUnicode fields are
        /// not present, and the values of the VolumeIDOffset and
        /// LocalBasePathOffset fields are zero. If the value of the
        /// LinkInfoHeaderSize field is greater than or equal to 0x00000024,
        /// the value of the LocalBasePathOffsetUnicode field is zero.
        const VOLUME_ID_AND_LOCAL_BASE_PATH = 0b0000_0000_0000_0000_0000_0000_0000_0001;

        /// If set, the CommonNetworkRelativeLink field is present, and its
        /// location is specified by the value of the
        /// CommonNetworkRelativeLinkOffset field.If not set, the
        /// CommonNetworkRelativeLink field is not present, and the value of
        /// the CommonNetworkRelativeLinkOffset field is zero
        const COMMON_NETWORK_RELATIVE_LINK_AND_PATH_SUFFIX = 0b0000_0000_0000_0000_0000_0000_0000_0010;
    }
}

binread_flags!(LinkInfoFlags, u32);

#[allow(missing_docs)]
impl LinkInfoFlags {
    pub fn has_volume_id_and_local_base_path(&self) -> bool {
        *self & Self::VOLUME_ID_AND_LOCAL_BASE_PATH == Self::VOLUME_ID_AND_LOCAL_BASE_PATH
    }

    pub fn has_common_network_relative_link_and_path_suffix(&self) -> bool {
        *self & Self::COMMON_NETWORK_RELATIVE_LINK_AND_PATH_SUFFIX == Self::COMMON_NETWORK_RELATIVE_LINK_AND_PATH_SUFFIX
    }
}

/// The VolumeID structure specifies information about the volume that a link
/// target was on when the link was created. This information is useful for
/// resolving the link if the file is not found in its original location.
#[derive(Clone, Debug, BinRead, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[get(get="pub")]
#[allow(unused)]
#[br(import(default_codepage: &'static Encoding))]
pub struct VolumeID {
    #[get(skip)]
    #[cfg_attr(feature = "serde", serde(skip))]
    start_offset: CurrentOffset,
    /// VolumeIDSize (4 bytes): A 32-bit, unsigned integer that specifies the
    /// size, in bytes, of this structure. This value MUST be greater than
    /// `0x00000010``. All offsets specified in this structure MUST be less
    /// than this value, and all strings contained in this structure MUST fit
    /// within the extent defined by this size.
    #[br(assert(volume_id_size > 0x10))]
    volume_id_size: u32,

    /// A 32-bit, unsigned integer that specifies the type of drive the link
    /// target is stored on.
    drive_type: DriveType,

    /// A 32-bit, unsigned integer that specifies the drive serial number of
    /// the volume the link target is stored on.
    drive_serial_number: u32,

    /// VolumeLabelOffset (4 bytes): A 32-bit, unsigned integer that
    /// specifies the location of a string that contains the volume label of
    /// the drive that the link target is stored on. This value is an offset,
    /// in bytes, from the start of the VolumeID structure to a NULL-terminated
    /// string of characters, defined by the system default code page. The
    /// volume label string is located in the Data field of this structure.
    ///
    /// If the value of this field is 0x00000014, it MUST be ignored, and the
    /// value of the VolumeLabelOffsetUnicode field MUST be used to locate the
    /// volume label string.
    #[br(assert(volume_label_offset < volume_id_size))]
    volume_label_offset: u32,

    /// VolumeLabelOffsetUnicode (4 bytes): An optional, 32-bit, unsigned
    /// integer that specifies the location of a string that contains the
    /// volume label of the drive that the link target is stored on. This value
    /// is an offset, in bytes, from the start of the VolumeID structure to a
    /// NULL-terminated string of Unicode characters. The volume label string
    /// is located in the Data field of this structure.
    ///
    /// If the value of the VolumeLabelOffset field is not 0x00000014, this
    /// field MUST NOT be present; instead, the value of the VolumeLabelOffset
    /// field MUST be used to locate the volume label string.

    #[br(if(volume_label_offset == 0x14))]
    volume_label_offset_unicode: Option<u32>,

    /// The label of the volume that the link target is stored on.
    #[br(
        args({volume_label_offset_unicode.and(Some(StringEncoding::Unicode)).unwrap_or(StringEncoding::CodePage(default_codepage))}),
        map=|s: NullTerminatedString| s.to_string()
    )]
    #[getset(skip)]
    volume_label: String,

    #[cfg_attr(feature = "serde", serde(skip))]
    _next_offset: CurrentOffset,
}

impl VolumeID {
    /// The label of the volume that the link target is stored on.
    pub fn volume_label(&self) -> &str {
        self.volume_label.as_ref()
    }
}

impl From<VolumeID> for Vec<u8> {
    fn from(_val: VolumeID) -> Self {
        unimplemented!()
    }
}

/// A 32-bit, unsigned integer that specifies the type of drive the link target is stored on.
#[derive(Clone, Debug, FromPrimitive, ToPrimitive, BinRead)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[br(repr(u32))]
pub enum DriveType {
    /// The drive type cannot be determined.
    DriveUnknown = 0x00,
    /// The root path is invalid; for example, there is no volume mounted at the path.
    DriveNoRootDir = 0x01,
    /// The drive has removable media, such as a floppy drive, thumb drive, or flash card reader.
    DriveRemovable = 0x02,
    /// The drive has fixed media, such as a hard drive or flash drive.
    DriveFixed = 0x03,
    /// The drive is a remote (network) drive.
    DriveRemote = 0x04,
    /// The drive is a CD-ROM drive.
    DriveCDRom = 0x05,
    /// The drive is a RAM disk.
    DriveRamdisk = 0x06,
}

/// The CommonNetworkRelativeLink structure specifies information about the network location where a
/// link target is stored, including the mapped drive letter and the UNC path prefix. For details on
/// UNC paths, see [MS-DFSNM] section 2.2.1.4.
///
/// <https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-shllink/23bb5877-e3dd-4799-9f50-79f05f938537>
#[derive(Clone, Debug, BinRead)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[allow(unused)]
#[br(import(default_codepage: &'static Encoding))]
pub struct CommonNetworkRelativeLink {
    #[serde(skip)]
    start_offset: CurrentOffset,

    /// CommonNetworkRelativeLinkSize (4 bytes): A 32-bit, unsigned integer
    /// that specifies the size, in bytes, of the CommonNetworkRelativeLink
    /// structure. This value MUST be greater than or equal to 0x00000014. All
    /// offsets specified in this structure MUST be less than this value, and
    /// all strings contained in this structure MUST fit within the extent
    /// defined by this size.
    #[br(assert(common_network_relative_link_size >= 0x14))]
    common_network_relative_link_size: u32,

    /// Flags that specify the contents of the DeviceNameOffset and
    /// NetProviderType fields.
    flags: CommonNetworkRelativeLinkFlags,

    /// NetNameOffset (4 bytes): A 32-bit, unsigned integer that specifies the
    /// location of the NetName field. This value is an offset, in bytes, from
    /// the start of the CommonNetworkRelativeLink structure.
    #[br(assert(net_name_offset < common_network_relative_link_size))]
    net_name_offset: u32,

    /// DeviceNameOffset (4 bytes): A 32-bit, unsigned integer that specifies
    /// the location of the DeviceName field. If the ValidDevice flag is set,
    /// this value is an offset, in bytes, from the start of the
    /// CommonNetworkRelativeLink structure; otherwise, this value MUST be
    /// zero.
    #[br(
        assert(
            device_name_offset < common_network_relative_link_size && 
            if flags.has_valid_device() {
                device_name_offset > 0
            } else {
                device_name_offset == 0
            }
        )
    )]
    device_name_offset: u32,

    /// NetworkProviderType (4 bytes): A 32-bit, unsigned integer that
    /// specifies the type of network provider. If the ValidNetType flag is
    /// set, this value MUST be one of the following; otherwise, this value
    /// MUST be ignored.
    #[br(map = |t| if flags.has_valid_net_type() {Some(t)} else {None})]
    network_provider_type: Option<NetworkProviderType>,

    /// NetNameOffsetUnicode (4 bytes): An optional, 32-bit, unsigned integer
    /// that specifies the location of the NetNameUnicode field. This value is
    /// an offset, in bytes, from the start of the CommonNetworkRelativeLink
    /// structure. This field MUST be present if the value of the NetNameOffset
    /// field is greater than 0x00000014; otherwise, this field MUST NOT be present.
    #[br(if(net_name_offset > 0x00000014))]
    net_name_offset_unicode: Option<u32>,

    /// DeviceNameOffsetUnicode (4 bytes): An optional, 32-bit, unsigned
    /// integer that specifies the location of the DeviceNameUnicode field.
    /// This value is an offset, in bytes, from the start of the
    /// CommonNetworkRelativeLink structure. This field MUST be present if the
    /// value of the NetNameOffset field is greater than 0x00000014; otherwise,
    /// this field MUST NOT be present.
    #[br(if(net_name_offset > 0x00000014))]
    device_name_offset_unicode: Option<u32>,

    /// A NULL–terminated string, as defined by the system default code
    /// page, which specifies a server share path; for example,
    /// "\\server\share".
    #[br(
        args(StringEncoding::CodePage(default_codepage)),
        map=|n: NullTerminatedString| n.to_string()
    )]
    net_name: String,

    /// A NULL–terminated string, as defined by the system default code
    /// page, which specifies a device; for example, the drive letter
    /// "D:".
    #[br(
        args(StringEncoding::CodePage(default_codepage)),
        map=|n: NullTerminatedString| n.to_string()
    )]
    device_name: String,

    /// An optional, NULL–terminated, Unicode string that is the Unicode
    /// version of the NetName string. This field MUST be present if the value
    /// of the NetNameOffset field is greater than 0x00000014; otherwise, this
    /// field MUST NOT be present.
    #[br(
        if(net_name_offset > 0x00000014),
        args(StringEncoding::Unicode),
        map=|n: NullTerminatedString| n.to_string()
    )]
    net_name_unicode: String,

    /// An optional, NULL–terminated, Unicode string that is the Unicode
    /// version of the DeviceName string. This field MUST be present if the
    /// value of the NetNameOffset field is greater than 0x00000014; otherwise,
    /// this field MUST NOT be present.
    #[br(
        if(net_name_offset > 0x00000014),
        args(StringEncoding::Unicode),
        map=|n: NullTerminatedString| n.to_string()
    )]
    device_name_unicode: String,
}

impl From<CommonNetworkRelativeLink> for Vec<u8> {
    fn from(_val: CommonNetworkRelativeLink) -> Self {
        unimplemented!()
    }
}

bitflags! {
    /// Flags that specify the contents of the DeviceNameOffset and NetProviderType fields.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    pub struct CommonNetworkRelativeLinkFlags: u32 {
        /// If set, the DeviceNameOffset field contains an offset to the device
        /// name. If not set, the DeviceNameOffset field does not contain an
        /// offset to the device name, and its value MUST be zero.
        const VALID_DEVICE = 0b0000_0000_0000_0000_0000_0000_0000_0001;
        /// If set, the NetProviderType field contains the network provider
        /// type. If not set, the NetProviderType field does not contain the
        /// network provider type, and its value MUST be zero.
        const VALID_NET_TYPE = 0b0000_0000_0000_0000_0000_0000_0000_0010;
    }
}

binread_flags!(CommonNetworkRelativeLinkFlags, u32);

#[allow(missing_docs)]
impl CommonNetworkRelativeLinkFlags {
    pub fn has_valid_device(&self) -> bool {
        *self & Self::VALID_DEVICE == Self::VALID_DEVICE
    }

    pub fn has_valid_net_type(&self) -> bool {
        *self & Self::VALID_NET_TYPE == Self::VALID_NET_TYPE
    }
}

/// A 32-bit, unsigned integer that specifies the type of network provider.
/// <https://learn.microsoft.com/de-de/windows/win32/api/winbase/ns-winbase-file_remote_protocol_info>
#[allow(missing_docs)]
#[derive(Clone, Debug, FromPrimitive, ToPrimitive, BinRead)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[br(repr(u32))]
pub enum NetworkProviderType {
    MSNet = 0x00010000,
    Smb = 0x00020000,
    Netware = 0x00030000,
    Vines = 0x00040000,
    TenNet = 0x00050000,
    Locus = 0x00060000,
    SunPCNFS = 0x00070000,
    LanStep = 0x00080000,
    NineTiles = 0x00090000,
    Lantastic = 0x000A0000,
    As400 = 0x000B0000,
    FTPNFS = 0x000C0000,
    PathWorks = 0x000D0000,
    LifeNet = 0x000E0000,
    PowerLAN = 0x000F0000,
    BWNFS = 0x00100000,
    Cogent = 0x00110000,
    Farallon = 0x00120000,
    AppleTalk = 0x00130000,
    Intergraph = 0x00140000,
    SymfoNet = 0x00150000,
    ClearCase = 0x00160000,
    Frontier = 0x00170000,
    BMC = 0x00180000,
    DCE = 0x00190000,
    Avid = 0x1a0000,
    Docuspace = 0x1b0000,
    Mangosoft = 0x1c0000,
    Sernet = 0x1d0000,
    Riverfront1 = 0x1e0000,
    Riverfront2 = 0x1f0000,
    Decorb = 0x200000,
    Protstor = 0x210000,
    FjRedir = 0x220000,
    Distinct = 0x230000,
    Twins = 0x240000,
    Rdr2Sample = 0x250000,
    CSC = 0x260000,
    _3In1 = 0x270000,
    ExtendNet = 0x290000,
    Stac = 0x2a0000,
    Foxbat = 0x2b0000,
    Yahoo = 0x2c0000,
    Exifs = 0x2d0000,
    Dav = 0x2e0000,
    Knoware = 0x2f0000,
    ObjectDire = 0x300000,
    Masfax = 0x310000,
    HobNfs = 0x320000,
    Shiva = 0x330000,
    Ibmal = 0x340000,
    Lock = 0x350000,
    Termsrv = 0x360000,
    Srt = 0x370000,
    Quincy = 0x380000,
    Openafs = 0x390000,
    Avid1 = 0x3a0000,
    Dfs = 0x3b0000,
    Kwnp = 0x3c0000,
    Zenworks = 0x3d0000,
    Driveonweb = 0x3e0000,
    Vmware = 0x3f0000,
    Rsfx = 0x400000,
    Mfiles = 0x410000,
    MsNfs = 0x420000,
    Google = 0x430000,
}
