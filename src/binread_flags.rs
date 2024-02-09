
macro_rules! binread_flags {
    ($type: ty, $repr:ty) => {
        impl binread::BinRead for $type {
            type Args = ();
        
            fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
                reader: &mut R,
                _options: &binread::ReadOptions,
                _args: Self::Args,
            ) -> binread::prelude::BinResult<Self> {
                use binread::BinReaderExt;
                let raw: $repr = reader.read_le()?;
                match Self::from_bits(raw) {
                    Some(res) => Ok(res),
                    None => Err(binread::Error::AssertFail {
                        pos: reader.stream_position()?,
                        message: format!("unable to convert '0x{raw:x}' to {}", stringify!($type))
                    })
                }
            }
        }
    };
}

pub(crate) use binread_flags;