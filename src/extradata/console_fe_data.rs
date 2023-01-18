use packed_struct::prelude::PackedStruct;

/// The ConsoleFEDataBlock structure specifies the code page to use
/// for displaying text when a link target specifies an application
/// that is run in a console window.
#[derive(Clone, Copy, Debug, PackedStruct)]
#[packed_struct(endian = "lsb")]
pub struct ConsoleFEDataBlock {
    /// A 32-bit, unsigned integer that specifies a code page language
    /// code identifier. For details concerning the structure and
    /// meaning of language code identifiers, see [MS-LCID].
    pub code_page: u32,
}
