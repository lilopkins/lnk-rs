use binread::BinRead;
use bitflags::bitflags;
use encoding_rs::UTF_16LE;
use getset::Getters;

use crate::{binread_flags::binread_flags, strings::FixedSizeString};

bitflags! {
  /// A 16-bit, unsigned integer that specifies the fill attributes that
  /// control the foreground and background text colors in the console
  /// window. The following bit definitions can be combined to specify 16
  /// different values each for the foreground and background colors:
  #[derive(Clone, Debug, Eq, PartialEq)]
  pub struct FillAttributeFlags: u16 {
    /// The foreground text color contains blue.
    const FOREGROUND_BLUE      = 0b0000_0000_0000_0001;
    /// The foreground text color contains green.
    const FOREGROUND_GREEN     = 0b0000_0000_0000_0010;
    /// The foreground text color contains red.
    const FOREGROUND_RED       = 0b0000_0000_0000_0100;
    /// The foreground text color is intensified.
    const FOREGROUND_INTENSITY = 0b0000_0000_0000_1000;

    /// The background text color contains blue.
    const BACKGROUND_BLUE      = 0b0000_0000_0001_0000;
    /// The background text color contains green.
    const BACKGROUND_GREEN     = 0b0000_0000_0010_0000;
    /// The background text color contains red.
    const BACKGROUND_RED       = 0b0000_0000_0100_0000;
    /// The background text color is intensified.
    const BACKGROUND_INTENSITY = 0b0000_0000_1000_0000;
  }
}

binread_flags!(FillAttributeFlags, u16);

bitflags! {
  /// A 32-bit, unsigned integer that specifies the family of the font
  /// used in the console window. This value MUST be comprised of a font
  /// family and an optional font pitch.
  #[derive(Clone, Debug, Eq, PartialEq)]
  pub struct FontFamilyFlags: u32 {
    /// The font family is unknown.
    const FF_DONT_CARE  = 0x0000;
    /// The font is variable-width with serifs; for example, "Times New Roman".
    const FF_ROMAN      = 0x0010;
    /// The font is variable-width without serifs; for example, "Arial".
    const FF_SWISS      = 0x0020;
    /// The font is fixed-width, with or without serifs; for example, "Courier New".
    const FF_MODERN     = 0x0030;
    /// The font is designed to look like handwriting; for example, "Cursive".
    const FF_SCRIPT     = 0x0040;
    /// The font is a novelty font; for example, "Old English".
    const FF_DECORATIVE = 0x0050;

    /// The font is a fixed-pitch font.
    const TMPF_FIXED_PITCH = 0x0001;
    /// The font is a vector font.
    const TMPF_VECTOR      = 0x0002;
    /// The font is a true-type font.
    const TMPF_TRUETYPE    = 0x0004;
    /// The font is specific to the device.
    const TMPF_DEVICE      = 0x0008;
  }
}

binread_flags!(FontFamilyFlags, u32);

/// The ConsoleDataBlock structure specifies the display settings to use
/// when a link target specifies an application that is run in a console
/// window.
#[derive(Clone, Debug, Getters, BinRead)]
#[get(get = "pub")]
#[allow(unused)]
pub struct ConsoleDataBlock {
    /// A 16-bit, unsigned integer that specifies the fill attributes that
    /// control the foreground and background text colors in the console
    /// window. The following bit definitions can be combined to specify 16
    /// different values each for the foreground and background colors:
    fill_attributes: FillAttributeFlags,
    /// A 16-bit, unsigned integer that specifies the fill attributes that
    /// control the foreground and background text color in the console
    /// window popup. The values are the same as for the FillAttributes
    /// field.
    popup_fill_attributes: FillAttributeFlags,
    /// A 16-bit, signed integer that specifies the horizontal size (X axis),
    /// in characters, of the console window buffer.
    screen_buffer_size_x: i16,
    /// A 16-bit, signed integer that specifies the vertical size (Y axis),
    /// in characters, of the console window buffer.
    screen_buffer_size_y: i16,
    /// A 16-bit, signed integer that specifies the horizontal size (X axis),
    /// in characters, of the console window.
    window_size_x: i16,
    /// A 16-bit, signed integer that specifies the vertical size (Y axis),
    /// in characters, of the console window.
    window_size_y: i16,
    /// A 16-bit, signed integer that specifies the horizontal coordinate (X axis),
    /// in pixels, of the console window origin.
    window_origin_x: i16,
    /// A 16-bit, signed integer that specifies the vertical coordinate (Y axis),
    /// in pixels, of the console window origin.
    window_origin_y: i16,

    #[getset(skip)]
    unused1: u32,

    #[getset(skip)]
    unused2: u32,

    /// A 32-bit, unsigned integer that specifies the size, in pixels, of the
    /// font used in the console window. The two most significant bytes contain
    /// the font height and the two least significant bytes contain the font
    /// width. For vector fonts, the width is set to zero.
    font_size: u32,
    /// A 32-bit, unsigned integer that specifies the family of the font used
    /// in the console window. This value MUST be comprised of a font family
    /// and an optional font pitch.
    font_family: FontFamilyFlags,
    /// A 32-bit, unsigned integer that specifies the stroke weight of the font
    /// used in the console window.
    font_weight: u32,
    /// A 32-character Unicode string that specifies the face name of the font
    /// used in the console window.
    #[br(args(64,UTF_16LE), map=|s:FixedSizeString| s.to_string())]
    face_name: String,
    /// A 32-bit, unsigned integer that specifies the size of the cursor, in
    /// pixels, used in the console window.
    cursor_size: u32,
    /// A 32-bit, unsigned integer that specifies whether to open the console
    /// window in full-screen mode.
    #[br(map=|b:u32| b != 0x00000000)]
    full_screen: bool,
    /// A 32-bit, unsigned integer that specifies whether to open the console
    /// window in QuikEdit mode. In QuickEdit mode, the mouse can be used to
    /// cut, copy, and paste text in the console window.
    #[br(map=|b:u32| b != 0x00000000)]
    quick_edit: bool,
    /// A 32-bit, unsigned integer that specifies insert mode in the console
    /// window.
    #[br(map=|b:u32| b != 0x00000000)]
    insert_mode: bool,
    /// A 32-bit, unsigned integer that specifies auto-position mode of the
    /// console window.
    #[br(map=|b:u32| b != 0x00000000)]
    auto_position: bool,
    /// A 32-bit, unsigned integer that specifies the size, in characters, of
    /// the buffer that is used to store a history of user input into the
    /// console window.
    history_buffer_size: u32,
    /// A 32-bit, unsigned integer that specifies the number of history
    /// buffers to use.
    number_of_history_buffers: u32,
    /// A 32-bit, unsigned integer that specifies whether to remove duplicates
    /// in the history buffer.
    #[br(map=|b:u32| b != 0x00000000)]
    history_no_dup: bool,
    /// A table of 16 32-bit, unsigned integers specifying the RGB colors that
    /// are used for text in the console window. The values of the fill
    /// attribute fields FillAttributes and PopupFillAttributes are used as
    /// indexes into this table to specify the final foreground and background
    /// color for a character.
    color_table: [u32; 16],
}
