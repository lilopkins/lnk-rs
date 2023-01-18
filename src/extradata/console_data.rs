use bitflags::bitflags;
use byteorder::{ByteOrder, LE};
use packed_struct::PackedStruct;

use crate::strings;

bitflags! {
  /// A 16-bit, unsigned integer that specifies the fill attributes that
  /// control the foreground and background text colors in the console
  /// window. The following bit definitions can be combined to specify 16
  /// different values each for the foreground and background colors:
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

impl PackedStruct for FillAttributeFlags {
    type ByteArray = [u8; 2];

    fn pack(&self) -> packed_struct::PackingResult<Self::ByteArray> {
        let mut dest = [0u8; 2];
        LE::write_u16(&mut dest, self.bits());
        Ok(dest)
    }

    fn unpack(src: &Self::ByteArray) -> packed_struct::PackingResult<Self> {
        let val = LE::read_u16(src);
        Ok(Self::from_bits_truncate(val))
    }
}

bitflags! {
  /// A 32-bit, unsigned integer that specifies the family of the font
  /// used in the console window. This value MUST be comprised of a font
  /// family and an optional font pitch.
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

impl PackedStruct for FontFamilyFlags {
    type ByteArray = [u8; 4];

    fn pack(&self) -> packed_struct::PackingResult<Self::ByteArray> {
        let mut dest = [0u8; 4];
        LE::write_u32(&mut dest, self.bits());
        Ok(dest)
    }

    fn unpack(src: &Self::ByteArray) -> packed_struct::PackingResult<Self> {
        let val = LE::read_u32(src);
        Ok(Self::from_bits_truncate(val))
    }
}

/// The ConsoleDataBlock structure specifies the display settings to use
/// when a link target specifies an application that is run in a console
/// window.
#[derive(Clone, Debug)]
pub struct ConsoleDataBlock {
    /// A 16-bit, unsigned integer that specifies the fill attributes that
    /// control the foreground and background text colors in the console
    /// window. The following bit definitions can be combined to specify 16
    /// different values each for the foreground and background colors:
    pub fill_attributes: FillAttributeFlags,
    /// A 16-bit, unsigned integer that specifies the fill attributes that
    /// control the foreground and background text color in the console
    /// window popup. The values are the same as for the FillAttributes
    /// field.
    pub popup_fill_attributes: FillAttributeFlags,
    /// A 16-bit, signed integer that specifies the horizontal size (X axis),
    /// in characters, of the console window buffer.
    pub screen_buffer_size_x: i16,
    /// A 16-bit, signed integer that specifies the vertical size (Y axis),
    /// in characters, of the console window buffer.
    pub screen_buffer_size_y: i16,
    /// A 16-bit, signed integer that specifies the horizontal size (X axis),
    /// in characters, of the console window.
    pub window_size_x: i16,
    /// A 16-bit, signed integer that specifies the vertical size (Y axis),
    /// in characters, of the console window.
    pub window_size_y: i16,
    /// A 16-bit, signed integer that specifies the horizontal coordinate (X axis),
    /// in pixels, of the console window origin.
    pub window_origin_x: i16,
    /// A 16-bit, signed integer that specifies the vertical coordinate (Y axis),
    /// in pixels, of the console window origin.
    pub window_origin_y: i16,
    /// A 32-bit, unsigned integer that specifies the size, in pixels, of the
    /// font used in the console window. The two most significant bytes contain
    /// the font height and the two least significant bytes contain the font
    /// width. For vector fonts, the width is set to zero.
    pub font_size: u32,
    /// A 32-bit, unsigned integer that specifies the family of the font used
    /// in the console window. This value MUST be comprised of a font family
    /// and an optional font pitch.
    pub font_family: FontFamilyFlags,
    /// A 32-bit, unsigned integer that specifies the stroke weight of the font
    /// used in the console window.
    pub font_weight: u32,
    /// A 32-character Unicode string that specifies the face name of the font
    /// used in the console window.
    pub face_name: String,
    /// A 32-bit, unsigned integer that specifies the size of the cursor, in
    /// pixels, used in the console window.
    pub cursor_size: u32,
    /// A 32-bit, unsigned integer that specifies whether to open the console
    /// window in full-screen mode.
    pub full_screen: bool,
    /// A 32-bit, unsigned integer that specifies whether to open the console
    /// window in QuikEdit mode. In QuickEdit mode, the mouse can be used to
    /// cut, copy, and paste text in the console window.
    pub quick_edit: bool,
    /// A 32-bit, unsigned integer that specifies insert mode in the console
    /// window.
    pub insert_mode: bool,
    /// A 32-bit, unsigned integer that specifies auto-position mode of the
    /// console window.
    pub auto_position: bool,
    /// A 32-bit, unsigned integer that specifies the size, in characters, of
    /// the buffer that is used to store a history of user input into the
    /// console window.
    pub history_buffer_size: u32,
    /// A 32-bit, unsigned integer that specifies the number of history
    /// buffers to use.
    pub number_of_history_buffers: u32,
    /// A 32-bit, unsigned integer that specifies whether to remove duplicates
    /// in the history buffer.
    pub history_no_dup: bool,
    /// A table of 16 32-bit, unsigned integers specifying the RGB colors that
    /// are used for text in the console window. The values of the fill
    /// attribute fields FillAttributes and PopupFillAttributes are used as
    /// indexes into this table to specify the final foreground and background
    /// color for a character.
    pub color_table: [u32; 16],
}

impl PackedStruct for ConsoleDataBlock {
    type ByteArray = [u8; 196];

    fn pack(&self) -> packed_struct::PackingResult<Self::ByteArray> {
        unimplemented!()
    }

    fn unpack(src: &Self::ByteArray) -> packed_struct::PackingResult<Self> {
        let fill_attributes = FillAttributeFlags::from_bits_truncate(LE::read_u16(src));
        let popup_fill_attributes =
            FillAttributeFlags::from_bits_truncate(LE::read_u16(&src[2..]));
        let screen_buffer_size_x = LE::read_i16(&src[4..]);
        let screen_buffer_size_y = LE::read_i16(&src[6..]);
        let window_size_x = LE::read_i16(&src[8..]);
        let window_size_y = LE::read_i16(&src[10..]);
        let window_origin_x = LE::read_i16(&src[12..]);
        let window_origin_y = LE::read_i16(&src[14..]);
        let font_size = LE::read_u32(&src[24..]);
        let font_family = FontFamilyFlags::from_bits_truncate(LE::read_u32(&src[28..]));
        let font_weight = LE::read_u32(&src[32..]);

        let mut string_data = [0u16; 32];
        LE::read_u16_into(&src[36..100], &mut string_data);
        let face_name =
            strings::trim_nul_terminated_string(String::from_utf16_lossy(&string_data).to_string());
        let cursor_size = LE::read_u32(&src[100..]);
        let full_screen = LE::read_u32(&src[104..]) != 0;
        let quick_edit = LE::read_u32(&src[108..]) != 0;
        let insert_mode = LE::read_u32(&src[112..]) != 0;
        let auto_position = LE::read_u32(&src[116..]) != 0;
        let history_buffer_size = LE::read_u32(&src[120..]);
        let number_of_history_buffers = LE::read_u32(&src[124..]);
        let history_no_dup = LE::read_u32(&src[128..]) != 0;
        let mut color_table = [0u32; 16];
        for idx in 0..16 {
            color_table[idx] = LE::read_u32(&src[(132 + idx * 4)..]);
        }
        Ok(Self {
            fill_attributes,
            popup_fill_attributes,
            screen_buffer_size_x,
            screen_buffer_size_y,
            window_size_x,
            window_size_y,
            window_origin_x,
            window_origin_y,
            font_size,
            font_family,
            font_weight,
            face_name,
            cursor_size,
            full_screen,
            quick_edit,
            insert_mode,
            auto_position,
            history_buffer_size,
            number_of_history_buffers,
            history_no_dup,
            color_table,
        })
    }
}
