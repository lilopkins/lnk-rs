use packed_struct::PackedStructSlice;

use crate::strings;

/// The DarwinDataBlock structure specifies an application identifier
/// that can be used instead of a link target IDList to install an
/// application when a shell link is activated.
#[derive(Clone, Debug)]
pub struct DarwinDataBlock {
    /// A NULL–terminated string, defined by the system default code
    /// page, which specifies an application identifier. This field
    /// SHOULD be ignored.
    pub darwin_data_ansi: String,
    /// An optional, NULL–terminated, Unicode string that specifies
    /// an application identifier.
    pub darwin_data_unicode: Option<String>,
}

impl PackedStructSlice for DarwinDataBlock {
    fn packed_bytes_size(_opt_self: Option<&Self>) -> packed_struct::PackingResult<usize> {
        unimplemented!()
    }

    fn pack_to_slice(&self, _output: &mut [u8]) -> packed_struct::PackingResult<()> {
        unimplemented!()
    }

    fn unpack_from_slice(src: &[u8]) -> packed_struct::PackingResult<Self> {
        let darwin_data_ansi =
            strings::trim_nul_terminated_string(String::from_utf8_lossy(&src[0..260]));
        let darwin_data_unicode_raw =
            strings::trim_nul_terminated_string(String::from_utf8_lossy(&src[260..520]));
        let darwin_data_unicode = if darwin_data_unicode_raw.len() == 0 {
            None
        } else {
            Some(darwin_data_unicode_raw)
        };
        Ok(Self {
            darwin_data_ansi,
            darwin_data_unicode,
        })
    }
}
