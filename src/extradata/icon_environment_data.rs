use packed_struct::PackedStructSlice;

use crate::strings;

/// The IconEnvironmentDataBlock structure specifies the path to an
/// icon. The path is encoded using environment variables, which makes
/// it possible to find the icon across machines where the locations
/// vary but are expressed using environment variables.
#[derive(Clone, Debug)]
pub struct IconEnvironmentDataBlock {
    /// A NULL-terminated string, defined by the system default code
    /// page, which specifies a path that is constructed with
    /// environment variables.
    pub target_ansi: String,
    /// An optional, NULL-terminated, Unicode string that specifies a
    /// path that is constructed with environment variables.
    pub target_unicode: Option<String>,
}

impl PackedStructSlice for IconEnvironmentDataBlock {
    fn packed_bytes_size(_opt_self: Option<&Self>) -> packed_struct::PackingResult<usize> {
        unimplemented!()
    }

    fn pack_to_slice(&self, _output: &mut [u8]) -> packed_struct::PackingResult<()> {
        unimplemented!()
    }

    fn unpack_from_slice(src: &[u8]) -> packed_struct::PackingResult<Self> {
        let target_ansi =
            strings::trim_nul_terminated_string(String::from_utf8_lossy(&src[0..260]));
        let target_unicode_raw =
            strings::trim_nul_terminated_string(String::from_utf8_lossy(&src[260..520]));
        let target_unicode = if target_unicode_raw.len() == 0 {
            None
        } else {
            Some(target_unicode_raw)
        };
        Ok(Self {
            target_ansi,
            target_unicode,
        })
    }
}
