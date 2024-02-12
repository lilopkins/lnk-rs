use binread::BinRead;
use encoding_rs::WINDOWS_1252;

use crate::{strings::FixedSizeString, Guid};

/// The TrackerDataBlock structure specifies data that can be used to
/// resolve a link target if it is not found in its original location
/// when the link is resolved. This data is passed to the Link
/// Tracking service [MS-DLTW] to find the link target.
#[derive(Clone, Debug, BinRead)]
#[allow(unused)]
pub struct TrackerDataBlock {
    /// A NULL–terminated character string, as defined by the system default
    /// code page, which specifies the NetBIOS name of the machine where
    /// the link target was last known to reside.
    #[br(args(16, WINDOWS_1252), map=|s:FixedSizeString| s.to_string())]
    machine_id: String,
    /// Two values in GUID packet representation ([MS-DTYP] section 2.3.4.2)
    /// that are used to find the link target with the Link Tracking service,
    /// as described in [MS-DLTW].
    droid: [Guid; 2],
    /// Two values in GUID packet representation that are used to find the
    /// link target with the Link Tracking service
    droid_birth: [Guid; 2],
}
