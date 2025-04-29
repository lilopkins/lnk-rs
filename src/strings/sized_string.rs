use binrw::{BinReaderExt, BinResult};
use encoding_rs::{Encoding, UTF_16LE};
use log::trace;

#[cfg(feature = "binwrite")]
use binrw::BinWrite;

use crate::{LinkFlags, StringEncoding};

/// reads a sized string from `reader` and converts it into a [`String`]
#[binrw::parser(reader: reader)]
pub fn parse_sized_string(
    link_flags: LinkFlags,
    expected_flag: LinkFlags,
    encoding: &'static Encoding,
) -> BinResult<Option<String>> {
    if link_flags.contains(expected_flag) {
        log::info!("reading string at {}", reader.stream_position()?);
        let count_characters: u16 = reader.read_le()?;
        trace!(
            "reading sized string of size '{count_characters}' at 0x{:08x}",
            reader.stream_position()?
        );

        let encoding = StringEncoding::from(link_flags, encoding);

        log::info!("characters: {count_characters}");

        match encoding {
            StringEncoding::CodePage(default_encoding) => {
                let mut buffer = vec![0; count_characters.into()];
                reader.read_exact(&mut buffer)?;
                let (cow, _, had_errors) = default_encoding.decode(&buffer);
                if had_errors {
                    return Err(binrw::error::Error::AssertFail {
                        pos: reader.stream_position()?,
                        message: format!(
                            "unable to decode String to CP1252 from buffer {buffer:?}"
                        ),
                    });
                }
                Ok(Some(cow.to_string()))
            }
            StringEncoding::Unicode => {
                let mut buffer = vec![0; (count_characters * 2).into()];
                reader.read_exact(&mut buffer)?;
                let (cow, _, had_errors) = UTF_16LE.decode(&buffer);
                if had_errors {
                    return Err(binrw::error::Error::AssertFail {
                        pos: reader.stream_position()?,
                        message: format!(
                            "unable to decode String to UTF-16LE from buffer {buffer:?}"
                        ),
                    });
                }
                Ok(Some(cow.to_string()))
            }
        }
    } else {
        Ok(None)
    }
}

/// converts a [`String`] to a sized string and writes it
#[cfg(feature = "binwrite")]
#[cfg_attr(feature="binwrite", binrw::writer(writer: writer))]
pub fn write_sized_string(
    s: &Option<String>,
    link_flags: LinkFlags,
    expected_flag: LinkFlags,
    encoding: &'static Encoding,
) -> BinResult<()> {
    if link_flags.contains(expected_flag) {
        assert!(s.is_some());
        let s = s.as_ref().expect("the flags indicate that there should be a value, but there is none");
        let count_characters = u16::try_from(s.len()).map_err(|_| binrw::Error::Custom {
            pos: writer.stream_position().unwrap(),
            err: Box::new("String is too long to be written"),
        })?;
        count_characters.write_le(writer)?;

        let encoding = StringEncoding::from(link_flags, encoding);
        match encoding {
            StringEncoding::CodePage(cp) => cp.encode(&s).0.write(writer)?,
            StringEncoding::Unicode => {
                let v: Vec<_> = s.encode_utf16().collect();
                v.write_le(writer)?
            }
        };
        Ok(())
    } else {
        assert!(s.is_none());
        Ok(())
    }
}
