use binread::{BinRead, BinReaderExt, NullWideString};
use encoding_rs::WINDOWS_1252;
use core::fmt::Display;

use crate::StringEncoding;

/// represents a string of unknown length which is NULL-terminated
#[derive(Clone, Debug)]
pub struct NullTerminatedString(String);

impl BinRead for NullTerminatedString {
    type Args = (StringEncoding,);

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        _options: &binread::ReadOptions,
        args: Self::Args,
    ) -> binread::prelude::BinResult<Self> {
        match args.0 {
            StringEncoding::CodePage => {
                let mut buffer = Vec::new();
                loop {
                    let c: u8 = reader.read_le()?;
                    if c == 0 {
                        break;
                    } else {
                        buffer.push(c);
                    }
                }
                let (cow, _, had_errors) = WINDOWS_1252.decode(&buffer);
                if had_errors {
                    return Err(binread::error::Error::AssertFail {
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