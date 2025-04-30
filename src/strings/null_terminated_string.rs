use binrw::{BinRead, BinReaderExt, NullWideString};
use core::fmt::Display;

use crate::StringEncoding;

/// represents a string of unknown length which is NULL-terminated
#[derive(Clone, Debug)]
pub struct NullTerminatedString(String);

impl BinRead for NullTerminatedString {
    type Args<'a> = (StringEncoding,);

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        match args.0 {
            StringEncoding::CodePage(default_codepage) => {
                let mut buffer = Vec::new();
                loop {
                    let c: u8 = reader.read_le()?;
                    if c == 0 {
                        break;
                    } else {
                        buffer.push(c);
                    }
                }
                let (cow, _, had_errors) = default_codepage.decode(&buffer);
                if had_errors {
                    return Err(binrw::error::Error::AssertFail {
                        pos: reader.stream_position()?,
                        message: format!(
                            "unable to decode String to CP1252 from buffer {buffer:?}"
                        ),
                    });
                }
                Ok(Self(cow.to_string()))
            }
            StringEncoding::Unicode => {
                let s: NullWideString = reader.read_le()?;
                Ok(Self(s.to_string()))
            }
        }
    }
}

impl Display for NullTerminatedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for NullTerminatedString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
