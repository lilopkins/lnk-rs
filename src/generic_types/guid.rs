use std::fmt::Display;

use binrw::{BinRead, BinWrite};
#[cfg(feature = "serde")]
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
    type Args<'a> = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let mut bytes = [0; 16];
        reader.read_exact(&mut bytes)?;
        let uuid = match endian {
            binrw::Endian::Big => Builder::from_bytes(bytes).into_uuid(),
            binrw::Endian::Little => Builder::from_bytes_le(bytes).into_uuid(),
        };
        Ok(Self(uuid))
    }
}

impl BinWrite for Guid {
    type Args<'a> = ();

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        let bytes = match endian {
            binrw::Endian::Big => *(self.0.as_bytes()),
            binrw::Endian::Little => {
                let (mut f1, mut f2, mut f3, f4) = self.0.to_fields_le();
                f1 = u32::from_le_bytes(f1.to_be_bytes());
                f2 = u16::from_le_bytes(f2.to_be_bytes());
                f3 = u16::from_le_bytes(f3.to_be_bytes());
                *(Builder::from_fields_le(f1, f2, f3, f4)
                    .into_uuid()
                    .as_bytes())
            }
        };
        writer.write_all(&bytes)?;
        Ok(())
    }
}

impl Display for Guid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Guid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use binrw::{BinReaderExt, BinWrite};
    use uuid::uuid;

    use super::Guid;

    #[test]
    fn test_guid_be() {
        let mut cursor = Cursor::new([0u8; 16]);
        let input = Guid(uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8"));

        input.write_be(&mut cursor).unwrap();
        cursor.set_position(0);
        let output: Guid = cursor.read_be().unwrap();
        assert_eq!(input, output);
    }

    #[test]
    fn test_guid_le() {
        let mut cursor = Cursor::new([0u8; 16]);
        let input = Guid(uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8"));

        input.write_le(&mut cursor).unwrap();
        cursor.set_position(0);
        let output: Guid = cursor.read_le().unwrap();
        assert_eq!(input, output);
    }
}
