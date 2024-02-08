
macro_rules! binread_flags {
    ($type: ty, $repr:ty) => {
        impl binread::BinRead for $type {
            type Args = ();
        
            fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
                reader: &mut R,
                options: &binread::ReadOptions,
                args: Self::Args,
            ) -> binread::prelude::BinResult<Self> {
                use binread::BinReaderExt;
                let raw: $repr = reader.read_le()?;
                Ok(Self::from_bits(raw).unwrap())
            }
        }
    };
}

pub(crate) use binread_flags;