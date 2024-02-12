use std::{fmt, mem::size_of};

#[cfg(feature="lnk2json")]
use serde::Serialize;

use binread::BinRead;
use getset::Getters;

/// A PropertyStoreDataBlock structure specifies a set of properties
/// that can be used by applications to store extra data in the
/// shell link.
/// TODO: implement <https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-propstore/1eb58eb3-e7d8-4a09-ac0e-8bcb14b6fa0e>
#[derive(Clone, BinRead, Getters)]
#[cfg_attr(feature = "lnk2json", derive(Serialize))]
#[br(import(block_size: u32), pre_assert(block_size >= 0x0000_000C))]
#[get(get="pub")]
#[allow(unused)]
pub struct PropertyStoreDataBlock {
    /// A serialized property storage structure ([MS-PROPSTORE] section 2.2).
    #[br(count=block_size - u32::try_from(2*size_of::<u32>()).unwrap())]
    property_store: Vec<u8>,
}

impl fmt::Debug for PropertyStoreDataBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PropertyStoreDataBlock {{ property_store: (serialized property storage structure) }}"
        )
    }
}
