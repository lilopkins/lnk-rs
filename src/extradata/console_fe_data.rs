use binread::BinRead;
use getset::Getters;

/// The ConsoleFEDataBlock structure specifies the code page to use
/// for displaying text when a link target specifies an application
/// that is run in a console window.
#[derive(Clone, Copy, Debug, BinRead, Getters)]
#[get(get="pub")]
#[allow(unused)]
pub struct ConsoleFEDataBlock {
    /// A 32-bit, unsigned integer that specifies a code page language
    /// code identifier. For details concerning the structure and
    /// meaning of language code identifiers, see [MS-LCID].
    code_page: u32,
}
