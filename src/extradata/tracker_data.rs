use byteorder::{ByteOrder, LE};
use packed_struct::prelude::PackedStruct;

use crate::strings;

/// The TrackerDataBlock structure specifies data that can be used to
/// resolve a link target if it is not found in its original location
/// when the link is resolved. This data is passed to the Link
/// Tracking service [MS-DLTW] to find the link target.
#[derive(Clone, Debug)]
pub struct TrackerDataBlock {
    /// A NULL–terminated character string, as defined by the system default
    /// code page, which specifies the NetBIOS name of the machine where
    /// the link target was last known to reside.
    pub machine_id: String,
    /// Two values in GUID packet representation ([MS-DTYP] section 2.3.4.2)
    /// that are used to find the link target with the Link Tracking service,
    /// as described in [MS-DLTW].
    pub droid: [u128; 2],
    /// Two values in GUID packet representation that are used to find the
    /// link target with the Link Tracking service
    pub droid_birth: [u128; 2],
}

impl PackedStruct for TrackerDataBlock {
    type ByteArray = [u8; 88];

    fn pack(&self) -> packed_struct::PackingResult<Self::ByteArray> {
        unimplemented!()
    }

    fn unpack(src: &Self::ByteArray) -> packed_struct::PackingResult<Self> {
        let machine_id =
            strings::trim_nul_terminated_string(String::from_utf8_lossy(&src[8..]).to_string());
        let droid_1 = LE::read_u128(&src[24..]);
        let droid_2 = LE::read_u128(&src[40..]);
        let droid_birth_1 = LE::read_u128(&src[56..]);
        let droid_birth_2 = LE::read_u128(&src[72..]);

        Ok(Self {
            machine_id,
            droid: [droid_1, droid_2],
            droid_birth: [droid_birth_1, droid_birth_2],
        })
    }
}
