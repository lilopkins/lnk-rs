use std::fmt;

use binrw::{BinRead, BinReaderExt, BinWrite};
use chrono::NaiveDateTime;

#[cfg(feature = "serde")]
use serde::Serialize;
use winstructs::timestamp::WinTimestamp;

/// The FILETIME structure is a 64-bit value that represents the number of
/// 100-nanosecond intervals that have elapsed since January 1, 1601,
/// Coordinated Universal Time (UTC).
#[derive(Clone)]
pub struct FileTime(WinTimestamp, u64);

impl fmt::Debug for FileTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.datetime())
    }
}

impl BinRead for FileTime {
    type Args<'a> = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let pos = reader.stream_position()?;
        let raw: u64 = reader.read_le()?;

        match WinTimestamp::new(&raw.to_le_bytes()) {
            Ok(timestamp) => Ok(Self(timestamp, raw)),
            Err(why) => Err(binrw::Error::AssertFail {
                pos,
                message: format!("{why}"),
            }),
        }
    }
}

impl BinWrite for FileTime {
    type Args<'a> = ();

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        _endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        self.1.write_le(writer)
    }
}

#[cfg(feature = "serde")]
impl Serialize for FileTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self.0))
    }
}

impl FileTime {
    /// Convert the `FileTime` object to a [[]]
    pub fn datetime(&self) -> NaiveDateTime {
        self.0.to_datetime().naive_utc()
    }

    /*
    /// Create a new `FileTime` object representing now.
    pub fn now() -> Self {
        Self::from(chrono::Local::now().naive_local())
    }
     */
}

impl Default for FileTime {
    fn default() -> Self {
        let raw = 0u64;
        let timestamp = WinTimestamp::new(&raw.to_le_bytes()).unwrap();
        Self(timestamp, raw)
    }
}

/*
impl From<NaiveDateTime> for FileTime {
    fn from(value: NaiveDateTime) -> Self {
        let duration = value - Self::epoch();
        let val = duration.num_microseconds().unwrap() * 10;
        let val = val as u64;
        Self::from(val)
    }
}
 */

impl From<FileTime> for u64 {
    fn from(val: FileTime) -> Self {
        val.1
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use binrw::{BinReaderExt, BinWrite};
    use winstructs::timestamp::WinTimestamp;

    use super::FileTime;

    #[test]
    fn test_guid_be() {
        let mut cursor = Cursor::new([0u8; 16]);
        let input = test_data();

        input.write_be(&mut cursor).unwrap();
        cursor.set_position(0);
        let output: FileTime = cursor.read_be().unwrap();
        assert_eq!(input.0.to_datetime(), output.0.to_datetime());
        assert_eq!(input.1, output.1);
    }

    #[test]
    fn test_guid_le() {
        let mut cursor = Cursor::new([0u8; 16]);
        let input = test_data();

        input.write_le(&mut cursor).unwrap();
        cursor.set_position(0);
        let output: FileTime = cursor.read_le().unwrap();
        assert_eq!(input.0.to_datetime(), output.0.to_datetime());
        assert_eq!(input.1, output.1);
    }

    fn test_data() -> FileTime {
        let raw = 123456789000u64;
        let timestamp = WinTimestamp::new(&raw.to_le_bytes()).unwrap();
        FileTime(timestamp, raw)
    }
}
