use std::fmt;

use packed_struct::PackedStructSlice;

/// A PropertyStoreDataBlock structure specifies a set of properties
/// that can be used by applications to store extra data in the
/// shell link.
#[derive(Clone)]
pub struct PropertyStoreDataBlock {
    /// A serialized property storage structure ([MS-PROPSTORE] section 2.2).
    pub property_store: Vec<u8>,
}

impl fmt::Debug for PropertyStoreDataBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PropertyStoreDataBlock {{ property_store: (serialized property storage structure) }}"
        )
    }
}

impl PackedStructSlice for PropertyStoreDataBlock {
    fn packed_bytes_size(_opt_self: Option<&Self>) -> packed_struct::PackingResult<usize> {
        unimplemented!()
    }

    fn pack_to_slice(&self, _output: &mut [u8]) -> packed_struct::PackingResult<()> {
        unimplemented!()
    }

    fn unpack_from_slice(src: &[u8]) -> packed_struct::PackingResult<Self> {
        Ok(Self {
            property_store: src.to_vec(),
        })
    }
}
