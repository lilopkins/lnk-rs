use binrw::BinRead;
use core::fmt::Display;
use encoding_rs::Encoding;
use substring::Substring;

/// represents a string that is stored in a buffer of a fixed size
#[derive(Clone, Debug)]
pub struct FixedSizeString(String);

impl BinRead for FixedSizeString {
    type Args<'a> = (usize, &'static Encoding);
    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let count = args.0;
        let encoding = args.1;
        let mut buffer = vec![0; count];
        reader.read_exact(&mut buffer)?;

        let (cow, _, had_errors) = encoding.decode(&buffer[..]);
        if had_errors {
            return Err(binrw::error::Error::AssertFail {
                pos: reader.stream_position()?,
                message: format!(
                    "unable to decode String to {} from buffer {buffer:?}",
                    encoding.name()
                ),
            });
        }
        let mut res = cow.to_string();
        if let Some(last_character) = res.find('\u{0000}') {
            res = res.substring(0, last_character).to_string();
        }
        Ok(Self(res))
    }
}

impl Display for FixedSizeString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for FixedSizeString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl FixedSizeString {
    /// returns `true` if the string is empty and `false` otherwise
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
