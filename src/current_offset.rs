use binread::{io::StreamPosition, BinRead};

/// implements [`BinRead`] by reading the current cursort position
/// and storing it as `u32`
#[derive(Clone, Debug)]
pub struct CurrentOffset(u32);

impl BinRead for CurrentOffset {
    type Args = ();

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        _options: &binread::ReadOptions,
        _args: Self::Args,
    ) -> binread::prelude::BinResult<Self> {
        Ok(Self(reader.stream_pos()?.try_into().expect("invalid offset")))
    }
}

impl AsRef<u32> for CurrentOffset {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}