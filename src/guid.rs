use std::fmt::Display;

use binread::BinRead;
#[cfg(feature="lnk2json")]
use serde::Serialize;
use uuid::{Builder, Uuid};

/// wraps a UUID
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Guid(Uuid);

impl From<Uuid> for Guid {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl BinRead for Guid {
    type Args = ();

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        _options: &binread::ReadOptions,
        _args: Self::Args,
    ) -> binread::prelude::BinResult<Self> {
        let mut bytes = [0; 16];
        reader.read_exact(&mut bytes)?;
        let uuid = Builder::from_bytes_le(bytes).into_uuid();
        Ok(Self(uuid))
    }
}

impl Display for Guid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature="lnk2json")]
impl Serialize for Guid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&self.0.to_string())
    }
}