macro_rules! binread_flags {
    ($type: ty, $repr:ty) => {
        impl binrw::BinRead for $type {
            type Args<'a> = ();

            fn read_options<R: std::io::Read + std::io::Seek>(
                reader: &mut R,
                endian: binrw::Endian,
                _args: Self::Args<'_>,
            ) -> binrw::BinResult<Self> {
                use binrw::BinReaderExt;
                let raw: $repr = match endian {
                    binrw::Endian::Big => reader.read_be()?,
                    binrw::Endian::Little => reader.read_le()?,
                };

                match Self::from_bits(raw) {
                    Some(res) => Ok(res),
                    None => Err(binrw::Error::AssertFail {
                        pos: reader.stream_position()?,
                        message: format!("unable to convert '0x{raw:x}' to {}", stringify!($type)),
                    }),
                }
            }
        }

        impl binrw::BinWrite for $type {
            type Args<'a> = ();

            fn write_options<W: std::io::Write + std::io::Seek>(
                &self,
                writer: &mut W,
                endian: binrw::Endian,
                _args: Self::Args<'_>,
            ) -> binrw::BinResult<()> {
                match endian {
                    binrw::Endian::Big => self.bits().write_be(writer),
                    binrw::Endian::Little => self.bits().write_le(writer),
                }
            }
        }
    };
}

pub(crate) use binread_flags;
