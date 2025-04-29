use binrw::BinRead;
use binrw::BinWrite;
use getset::{Getters, MutGetters, Setters};
use num_derive::FromPrimitive;

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::FileTime;
use crate::Guid;

mod hotkey_flags;
pub use hotkey_flags::{HotkeyFlags, HotkeyKey, HotkeyModifiers};

mod link_flags;
pub use link_flags::LinkFlags;

mod file_attributes_flags;
pub use file_attributes_flags::FileAttributeFlags;

/// A ShellLinkHeader structure (section 2.1), which contains identification
/// information, timestamps, and flags that specify the presence of optional
/// structures.
#[derive(Clone, Debug, Getters, MutGetters, Setters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(BinRead)]
#[cfg_attr(feature = "binwrite", derive(BinWrite))]
// #[br(little)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct ShellLinkHeader {
    /// The size, in bytes, of this structure. This value MUST be 0x0000004C.
    #[br(assert(header_size == 0x0000_004c))]
    header_size: u32,

    /// This value MUST be 00021401-0000-0000-C000-000000000046.
    #[br(assert(link_clsid == Guid::from(uuid::uuid!("00021401-0000-0000-C000-000000000046"))))]
    link_clsid: Guid,

    /// A LinkFlags structure (section 2.1.1) that specifies information about the shell link and
    /// the presence of optional portions of the structure.
    link_flags: LinkFlags,

    /// A FileAttributesFlags structure (section 2.1.2) that specifies information about the link
    /// target.
    file_attributes: FileAttributeFlags,

    /// A FILETIME structure ([MS-DTYP]section 2.3.3) that specifies the creation time of the link
    /// target in UTC (Coordinated Universal Time). If the value is zero, there is no creation time
    /// set on the link target.
    creation_time: FileTime,

    /// A FILETIME structure ([MS-DTYP] section2.3.3) that specifies the access time of the link
    /// target in UTC (Coordinated Universal Time). If the value is zero, there is no access time
    /// set on the link target.
    access_time: FileTime,

    /// A FILETIME structure ([MS-DTYP] section 2.3.3) that specifies the write time of the link
    /// target in UTC (Coordinated Universal Time). If the value is zero, there is no write time
    /// set on the link target.
    write_time: FileTime,

    /// A 32-bit unsigned integer that specifies the size, in bytes, of the link target. If the
    /// link target fileis larger than 0xFFFFFFFF, this value specifies the least significant 32
    /// bits of the link target file size.
    file_size: u32,

    /// A 32-bit signed integer that specifies the index of an icon within a given icon location.
    icon_index: i32,

    /// A 32-bit unsigned integer that specifies the expected window state of an application
    /// launched by the link.
    show_command: ShowCommand,

    /// A HotkeyFlags structure (section 2.1.3) that specifies the keystrokes used to launch the
    /// application referenced by the shortcut key. This value is assigned to the application after
    /// it is launched, so that pressing the key activates that application.
    hotkey: HotkeyFlags,

    /// A value that MUST be zero
    #[br(assert(reserved1 == 0))]
    #[cfg_attr(feature = "serde", serde(skip))]
    reserved1: u16,

    /// A value that MUST be zero
    #[br(assert(reserved2 == 0))]
    #[cfg_attr(feature = "serde", serde(skip))]
    reserved2: u32,

    /// A value that MUST be zero
    #[br(assert(reserved3 == 0))]
    #[cfg_attr(feature = "serde", serde(skip))]
    reserved3: u32,
}

impl ShellLinkHeader {
    /// Set some link flags
    pub fn update_link_flags(&mut self, link_flags: LinkFlags, value: bool) {
        self.link_flags.set(link_flags, value);
    }
}

impl Default for ShellLinkHeader {
    /// Create a new, blank, ShellLinkHeader
    fn default() -> Self {
        Self {
            header_size: 0x4c,
            link_clsid: Guid::from(uuid::uuid!("00021401-0000-0000-C000-000000000046")),
            link_flags: LinkFlags::IS_UNICODE,
            file_attributes: FileAttributeFlags::FILE_ATTRIBUTE_NORMAL,
            creation_time: FileTime::default(),
            access_time: FileTime::default(),
            write_time: FileTime::default(),
            file_size: 0,
            icon_index: 0,
            show_command: ShowCommand::ShowNormal,
            hotkey: HotkeyFlags::new(HotkeyKey::NoKeyAssigned, HotkeyModifiers::NO_MODIFIER),
            reserved1: 0,
            reserved2: 0,
            reserved3: 0,
        }
    }
}

/// The expected window state of an application launched by the link.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, FromPrimitive, BinRead, BinWrite)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[brw(repr=u32)]
pub enum ShowCommand {
    /// The application is open and its window is open in a normal fashion.
    ShowNormal = 0x01,
    /// The application is open, and keyboard focus is given to the application, but its window is
    /// not shown.
    ShowMaximized = 0x03,
    /// The application is open, but its window is not shown. It is not given the keyboard focus.
    ShowMinNoActive = 0x07,
}
