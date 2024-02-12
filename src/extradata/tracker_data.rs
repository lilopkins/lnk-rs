use binread::BinRead;
use encoding_rs::Encoding;

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{strings::FixedSizeString, Guid};

/// The TrackerDataBlock structure specifies data that can be used to
/// resolve a link target if it is not found in its original location
/// when the link is resolved. This data is passed to the Link
/// Tracking service [MS-DLTW] to find the link target.
#[derive(Clone, Debug, BinRead)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[br(import(block_size: u32, default_codepage: &'static Encoding), pre_assert(block_size == 0x0000_00060))]
#[allow(unused)]
pub struct TrackerDataBlock {
    /// A 32-bit, unsigned integer that specifies the size of the rest of the
    /// TrackerDataBlock structure, including this Length field. This value
    /// MUST be 0x00000058.
    #[br(assert(length == 0x00000058))]
    length: u32,

    /// A 32-bit, unsigned integer. This value MUST be 0x00000000   
    #[br(assert(version == 0x00000000))]
    version: u32,

    /// A NULL–terminated character string, as defined by the system default
    /// code page, which specifies the NetBIOS name of the machine where
    /// the link target was last known to reside.
    #[br(args(16, default_codepage), map=|s:FixedSizeString| s.to_string())]
    machine_id: String,
    /// Two values in GUID packet representation ([MS-DTYP] section 2.3.4.2)
    /// that are used to find the link target with the Link Tracking service,
    /// as described in [MS-DLTW].
    droid: [Guid; 2],
    /// Two values in GUID packet representation that are used to find the
    /// link target with the Link Tracking service
    droid_birth: [Guid; 2],
}
