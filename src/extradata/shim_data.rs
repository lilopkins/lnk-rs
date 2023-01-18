use packed_struct::PackedStructSlice;

/// The ShimDataBlock structure specifies the name of a shim that can
/// be applied when activating a link target.
#[derive(Clone, Debug)]
pub struct ShimDataBlock {
    /// A Unicode string that specifies the name of a shim layer to apply
    /// to a link target when it is being activated.
    pub layer_name: String,
}

impl PackedStructSlice for ShimDataBlock {
    fn packed_bytes_size(_opt_self: Option<&Self>) -> packed_struct::PackingResult<usize> {
        unimplemented!()
    }

    fn pack_to_slice(&self, _output: &mut [u8]) -> packed_struct::PackingResult<()> {
        unimplemented!()
    }

    fn unpack_from_slice(src: &[u8]) -> packed_struct::PackingResult<Self> {
        let layer_name = String::from_utf8_lossy(src).to_string();
        Ok(Self { layer_name })
    }
}
