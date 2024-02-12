use std::io::SeekFrom;

use binread::BinRead;
use bitflags::bitflags;
use getset::Getters;
use num_derive::{FromPrimitive, ToPrimitive};

use crate::{
    binread_flags::binread_flags,
    strings::{NullTerminatedString, StringEncoding},
    CurrentOffset,
};

#[cfg(feature="lnk2json")]
use serde::Serialize;

/// The LinkInfo structure specifies information necessary to resolve a
/// linktarget if it is not found in its original location. This includes
/// information about the volume that the target was stored on, the mapped
/// drive letter, and a Universal Naming Convention (UNC)form of the path
/// if one existed when the linkwas created. For more details about UNC
/// paths, see [MS-DFSNM] section 2.2.1.4
#[derive(Debug, BinRead, Getters)]
#[cfg_attr(feature = "lnk2json", derive(Serialize))]
#[get(get="pub")]
#[allow(unused)]
pub struct LinkInfo {
    #[serde(skip)]
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
    #[br(assert(if link_info_flags & LinkInfoFlags::VOLUME_ID_AND_LOCAL_BASE_PATH == LinkInfoFlags::VOLUME_ID_AND_LOCAL_BASE_PATH{volume_id_offset > 0 && volume_id_offset < link_info_size} else {volume_id_offset == 0}))]
    volume_id_offset: u32,

    /// LocalBasePathOffset (4 bytes): A 32-bit, unsigned integer that
    /// specifies the location of the LocalBasePath field. If the
    /// VolumeIDAndLocalBasePath flag is set, this value is an offset, in
    /// bytes, from the start of the LinkInfo structure; otherwise, this value
    /// MUST be zero.
    #[br(assert(if link_info_flags & LinkInfoFlags::VOLUME_ID_AND_LOCAL_BASE_PATH == LinkInfoFlags::VOLUME_ID_AND_LOCAL_BASE_PATH{local_base_path_offset > 0 && local_base_path_offset < link_info_size} else {local_base_path_offset == 0}))]
    local_base_path_offset: u32,

    /// CommonNetworkRelativeLinkOffset (4 bytes): A 32-bit, unsigned integer
    /// that specifies the location of the CommonNetworkRelativeLink field. If
    /// the CommonNetworkRelativeLinkAndPathSuffix flag is set, this value is
    /// an offset, in bytes, from the start of the LinkInfo structure;
    /// otherwise, this value MUST be zero.
    #[br(assert(if link_info_flags & LinkInfoFlags::COMMON_NETWORK_RELATIVE_LINK_AND_PATH_SUFFIX == LinkInfoFlags::COMMON_NETWORK_RELATIVE_LINK_AND_PATH_SUFFIX{common_network_relative_link_offset > 0 && common_network_relative_link_offset < link_info_size} else {common_network_relative_link_offset == 0}))]
    common_network_relative_link_offset: u32,

    /// CommonPathSuffixOffset (4 bytes): A 32-bit, unsigned integer that
    /// specifies the location of the CommonPathSuffix field. This value is
    /// an offset, in bytes, from the start of the LinkInfo structure.
    #[br(assert(common_path_suffix_offset < link_info_size))]
    common_path_suffix_offset: u32,

    /// LocalBasePathOffsetUnicode (4 bytes): An optional, 32-bit, unsigned
    /// integer that specifies the location of the LocalBasePathUnicode field.
    /// If the VolumeIDAndLocalBasePath flag is set, this value is an offset,
    /// in bytes, from the start of the LinkInfo structure; otherwise, this
    /// value MUST be zero. This field can be present only if the value of the
    /// LinkInfoHeaderSize field is greater than or equal to 0x00000024.
    #[br(
        if(link_info_header_size >= 0x24),
        assert(if let Some(offset) = local_base_path_offset_unicode{ if link_info_flags & LinkInfoFlags::VOLUME_ID_AND_LOCAL_BASE_PATH == LinkInfoFlags::VOLUME_ID_AND_LOCAL_BASE_PATH{offset > 0 && offset < link_info_size} else {false}} else {true}))]
    local_base_path_offset_unicode: Option<u32>,

    /// CommonPathSuffixOffsetUnicode (4 bytes): An optional, 32-bit, unsigned
    /// integer that specifies the location of the CommonPathSuffixUnicode
    /// field. This value is an offset, in bytes, from the start of the
    /// LinkInfo structure. This field can be present only if the value of the
    /// LinkInfoHeaderSize field is greater than or equal to 0x00000024.
    #[br(
        if(link_info_header_size >= 0x24),
        assert(if let Some(offset) = common_path_suffix_offset_unicode{ if link_info_flags & LinkInfoFlags::COMMON_NETWORK_RELATIVE_LINK_AND_PATH_SUFFIX == LinkInfoFlags::COMMON_NETWORK_RELATIVE_LINK_AND_PATH_SUFFIX{offset > 0 && offset < link_info_size} else {false}} else {true}))]
    common_path_suffix_offset_unicode: Option<u32>,

    /// An optional VolumeID structure (section 2.3.1) that specifies
    /// information about the volume that the link target was on when the link
    /// was created. This field is present if the VolumeIDAndLocalBasePath
    /// flag is set.
    #[br(seek_before(SeekFrom::Start((*start_offset.as_ref() + volume_id_offset).into())),
        if(link_info_flags & LinkInfoFlags::VOLUME_ID_AND_LOCAL_BASE_PATH == LinkInfoFlags::VOLUME_ID_AND_LOCAL_BASE_PATH))]
    volume_id: Option<VolumeID>,

    /// An optional, NULL–terminated string, defined by the system default code
    /// page, which is used to construct the full path to the link item or link
    /// target by appending the string in the CommonPathSuffix field. This
    /// field is present if the VolumeIDAndLocalBasePath flag is set.
    #[br(
        seek_before(SeekFrom::Start((*start_offset.as_ref() + local_base_path_offset_unicode.unwrap_or(local_base_path_offset)).into())),
        if(link_info_flags & LinkInfoFlags::VOLUME_ID_AND_LOCAL_BASE_PATH == LinkInfoFlags::VOLUME_ID_AND_LOCAL_BASE_PATH),
        args({local_base_path_offset_unicode.and(Some(StringEncoding::Unicode)).unwrap_or(StringEncoding::CodePage)}),
        map=|o: Option<NullTerminatedString>| o.map(|n| n.to_string())
    )]
    #[getset(skip)]
    local_base_path: Option<String>,

    /// An optional CommonNetworkRelativeLink structure (section 2.3.2) that
    /// specifies information about the network location where the link target
    /// is stored.
    #[br(seek_before(SeekFrom::Start((*start_offset.as_ref() + common_network_relative_link_offset).into())),
        if(link_info_flags & LinkInfoFlags::COMMON_NETWORK_RELATIVE_LINK_AND_PATH_SUFFIX == LinkInfoFlags::COMMON_NETWORK_RELATIVE_LINK_AND_PATH_SUFFIX))]
    common_network_relative_link: Option<CommonNetworkRelativeLink>,
    
    /// A NULL–terminated string, defined by the system default code page,
    /// which is used to construct the full path to the link item or link
    /// target by being appended to the string in the LocalBasePath field.
    #[br(
        seek_before(SeekFrom::Start((*start_offset.as_ref() + common_path_suffix_offset_unicode.unwrap_or(common_path_suffix_offset)).into())),
        args({common_path_suffix_offset_unicode.and(Some(StringEncoding::Unicode)).unwrap_or(StringEncoding::CodePage)}),
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
        if(link_info_flags & LinkInfoFlags::VOLUME_ID_AND_LOCAL_BASE_PATH == LinkInfoFlags::VOLUME_ID_AND_LOCAL_BASE_PATH),
        seek_before(SeekFrom::Start((*start_offset.as_ref() + local_base_path_offset_unicode.unwrap_or(local_base_path_offset)).into())),
        args({local_base_path_offset_unicode.and(Some(StringEncoding::Unicode)).unwrap_or(StringEncoding::CodePage)}),
        map=|o: Option<NullTerminatedString>| o.map(|n| n.to_string())
    )]
    local_base_path_unicode: Option<String>,
    
    #[br(seek_before(SeekFrom::Start((*start_offset.as_ref() + link_info_size).into())))]
    #[serde(skip)]
    _next_offset: CurrentOffset,
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

/*
impl Default for LinkInfo {
    fn default() -> Self {
        Self {
            size: 0,
            _link_info_flags: LinkInfoFlags::empty(),
            volume_id: None,
            local_base_path: None,
            common_network_relative_link: None,
            common_path_suffix: String::new(),
            local_base_path_unicode: None,
            common_path_suffix_unicode: None,
        }
    }
}
impl From<&[u8]> for LinkInfo {
    fn from(data: &[u8]) -> Self {
        let mut link_info = Self::default();

        link_info.size = LE::read_u32(data);
        let header_size = LE::read_u32(&data[4..]);
        let extra_offsets_specified = header_size >= 0x24;
        let flags = LinkInfoFlags::from_bits_truncate(LE::read_u32(&data[8..]));
        let volume_id_offset = LE::read_u32(&data[12..]) as usize;
        let local_base_path_offset = LE::read_u32(&data[16..]) as usize;
        let common_network_relative_link_offset = LE::read_u32(&data[20..]) as usize;
        let common_path_suffix_offset = LE::read_u32(&data[24..]) as usize;
        let mut local_base_path_offset_unicode = 0;
        if extra_offsets_specified {
            local_base_path_offset_unicode = LE::read_u32(&data[28..]) as usize;
            let common_path_suffix_offset_unicode = LE::read_u32(&data[32..]) as usize;

            if common_path_suffix_offset_unicode != 0 {
                link_info.common_path_suffix_unicode = Some(strings::trim_nul_terminated_string(
                    String::from_utf8_lossy(&data[common_path_suffix_offset_unicode..]).to_string(),
                ));
            }
        }
        if flags & LinkInfoFlags::VOLUME_ID_AND_LOCAL_BASE_PATH
            == LinkInfoFlags::VOLUME_ID_AND_LOCAL_BASE_PATH
        {
            assert_ne!(volume_id_offset, 0);
            assert_ne!(local_base_path_offset, 0);
            link_info.volume_id = Some(VolumeID::from(&data[volume_id_offset..]));
            link_info.local_base_path = Some(strings::trim_nul_terminated_string(
                String::from_utf8_lossy(&data[local_base_path_offset..]).to_string(),
            ));

            if local_base_path_offset_unicode != 0 {
                link_info.local_base_path_unicode = Some(strings::trim_nul_terminated_string(
                    String::from_utf8_lossy(&data[local_base_path_offset_unicode..]).to_string(),
                ));
            }
        }
        if flags & LinkInfoFlags::COMMON_NETWORK_RELATIVE_LINK_AND_PATH_SUFFIX
            == LinkInfoFlags::COMMON_NETWORK_RELATIVE_LINK_AND_PATH_SUFFIX
        {
            assert_ne!(common_network_relative_link_offset, 0);
            link_info.common_network_relative_link = Some(CommonNetworkRelativeLink::from(
                &data[common_network_relative_link_offset..],
            ));
        }
        link_info.common_path_suffix = strings::trim_nul_terminated_string(
            String::from_utf8_lossy(&data[common_path_suffix_offset..]).to_string(),
        );

        link_info
    }
}
 */

impl From<LinkInfo> for Vec<u8> {
    fn from(_val: LinkInfo) -> Self {
        unimplemented!()
    }
}

bitflags! {
    /// Flags that specify whether the VolumeID, LocalBasePath, LocalBasePathUnicode,
    /// and CommonNetworkRelativeLink fields are present in this structure.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "lnk2json", derive(Serialize))]
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

/// The VolumeID structure specifies information about the volume that a link
/// target was on when the link was created. This information is useful for
/// resolving the link if the file is not found in its original location.
#[derive(Clone, Debug, BinRead, Getters)]
#[cfg_attr(feature = "lnk2json", derive(Serialize))]
#[get(get="pub")]
#[allow(unused)]
pub struct VolumeID {
    #[get(skip)]
    #[serde(skip)]
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
        seek_before(SeekFrom::Start((*start_offset.as_ref() + volume_label_offset_unicode.unwrap_or(volume_label_offset)).into())),
        args({volume_label_offset_unicode.and(Some(StringEncoding::Unicode)).unwrap_or(StringEncoding::CodePage)}),
        map=|s: NullTerminatedString| s.to_string()
    )]
    #[getset(skip)]
    volume_label: String,

    #[br(seek_before(SeekFrom::Start((*start_offset.as_ref() + volume_id_size).into())))]
    #[serde(skip)]
    _next_offset: CurrentOffset,
}

impl VolumeID {
    /// The label of the volume that the link target is stored on.
    pub fn volume_label(&self) -> &str {
        self.volume_label.as_ref()
    }
}
/*
impl From<&[u8]> for VolumeID {
    fn from(data: &[u8]) -> Self {
        let mut volume_id = VolumeID::default();

        let _size = LE::read_u32(data);
        volume_id.drive_type = DriveType::from_u32(LE::read_u32(&data[4..])).unwrap();
        volume_id.drive_serial_number = LE::read_u32(&data[8..]);
        let mut volume_label_offset = LE::read_u32(&data[12..]) as usize;
        if volume_label_offset == 0x14 {
            volume_label_offset /* _unicode */ = LE::read_u32(&data[16..]) as usize;
        }
        volume_id.volume_label = strings::trim_nul_terminated_string(
            String::from_utf8_lossy(&data[volume_label_offset..]).to_string(),
        );

        volume_id
    }
}
 */

impl From<VolumeID> for Vec<u8> {
    fn from(_val: VolumeID) -> Self {
        unimplemented!()
    }
}

/// A 32-bit, unsigned integer that specifies the type of drive the link target is stored on.
#[derive(Clone, Debug, FromPrimitive, ToPrimitive, BinRead)]
#[cfg_attr(feature = "lnk2json", derive(Serialize))]
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
#[cfg_attr(feature = "lnk2json", derive(Serialize))]
#[allow(unused)]
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
    #[br(assert(device_name_offset < common_network_relative_link_size))]
    device_name_offset: u32,

    /// NetworkProviderType (4 bytes): A 32-bit, unsigned integer that
    /// specifies the type of network provider. If the ValidNetType flag is
    /// set, this value MUST be one of the following; otherwise, this value
    /// MUST be ignored.
    #[br(map = |t| if flags & CommonNetworkRelativeLinkFlags::VALID_NET_TYPE == CommonNetworkRelativeLinkFlags::VALID_NET_TYPE {Some(t)} else {None})]
    network_provider_type: Option<NetworkProviderType>,

    /// NetNameOffsetUnicode (4 bytes): An optional, 32-bit, unsigned integer
    /// that specifies the location of the NetNameUnicode field. This value is
    /// an offset, in bytes, from the start of the CommonNetworkRelativeLink
    /// structure. This field MUST be present if the value of the NetNameOffset
    /// field is greater than 0x00000014; otherwise, this field MUST NOT be present.
    #[br(if(net_name_offset > 0x14))]
    net_name_offset_unicode: Option<u32>,

    /// DeviceNameOffsetUnicode (4 bytes): An optional, 32-bit, unsigned
    /// integer that specifies the location of the DeviceNameUnicode field.
    /// This value is an offset, in bytes, from the start of the
    /// CommonNetworkRelativeLink structure. This field MUST be present if the
    /// value of the NetNameOffset field is greater than 0x00000014; otherwise,
    /// this field MUST NOT be present.
    #[br(if(net_name_offset > 0x14))]
    device_name_offset_unicode: Option<u32>,

    /// A NULL–terminated string, as defined by the system default code
    /// page, which specifies a server share path; for example,
    /// "\\server\share".
    #[br(
        seek_before(SeekFrom::Start((*start_offset.as_ref() + net_name_offset_unicode.unwrap_or(net_name_offset)).into())),
        args({net_name_offset_unicode.and(Some(StringEncoding::Unicode)).unwrap_or(StringEncoding::CodePage)}),
        map=|n: NullTerminatedString| n.to_string()
    )]
    net_name: String,

    /// A NULL–terminated string, as defined by the system default code
    /// page, which specifies a device; for example, the drive letter
    /// "D:".
    #[br(
        seek_before(SeekFrom::Start((*start_offset.as_ref() + device_name_offset_unicode.unwrap_or(device_name_offset)).into())),
        args({device_name_offset_unicode.and(Some(StringEncoding::Unicode)).unwrap_or(StringEncoding::CodePage)}),
        map=|n: NullTerminatedString| n.to_string()
    )]
    device_name: String,

    #[br(seek_before(SeekFrom::Start((*start_offset.as_ref() + common_network_relative_link_size).into())))]
    #[serde(skip)]
    _next_offset: CurrentOffset,
}
/*
impl From<&[u8]> for CommonNetworkRelativeLink {
    fn from(data: &[u8]) -> Self {
        let mut link = CommonNetworkRelativeLink::default();

        let size = LE::read_u32(data);
        assert!(size >= 0x14);
        link.flags = CommonNetworkRelativeLinkFlags::from_bits_truncate(LE::read_u32(&data[4..]));
        let net_name_offset = LE::read_u32(&data[8..]) as usize;
        let device_name_offset = LE::read_u32(&data[12..]) as usize;
        if link.flags & CommonNetworkRelativeLinkFlags::VALID_NET_TYPE
            == CommonNetworkRelativeLinkFlags::VALID_NET_TYPE
        {
            link.network_provider_type = NetworkProviderType::from_u32(LE::read_u32(&data[16..]));
        }
        link.net_name = strings::trim_nul_terminated_string(
            String::from_utf8_lossy(&data[net_name_offset..]).to_string(),
        );
        link.device_name = strings::trim_nul_terminated_string(
            String::from_utf8_lossy(&data[device_name_offset..]).to_string(),
        );
        if net_name_offset >= 0x14 {
            let net_name_offset_unicode = LE::read_u32(&data[20..]) as usize;
            let device_name_offset_unicode = LE::read_u32(&data[24..]) as usize;
            link.net_name_unicode = Some(strings::trim_nul_terminated_string(
                String::from_utf8_lossy(&data[net_name_offset_unicode..]).to_string(),
            ));
            link.device_name_unicode = Some(strings::trim_nul_terminated_string(
                String::from_utf8_lossy(&data[device_name_offset_unicode..]).to_string(),
            ));
        }

        link
    }
}
 */

impl From<CommonNetworkRelativeLink> for Vec<u8> {
    fn from(_val: CommonNetworkRelativeLink) -> Self {
        unimplemented!()
    }
}

bitflags! {
    /// Flags that specify the contents of the DeviceNameOffset and NetProviderType fields.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "lnk2json", derive(Serialize))]
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

/// A 32-bit, unsigned integer that specifies the type of network provider.
#[allow(missing_docs)]
#[derive(Clone, Debug, FromPrimitive, ToPrimitive, BinRead)]
#[cfg_attr(feature = "lnk2json", derive(Serialize))]
#[br(repr(u32))]
pub enum NetworkProviderType {
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
