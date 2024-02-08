use binread::{io::StreamPosition, BinRead};

#[derive(Clone, Debug)]
pub struct CurrentOffset(u32);

impl BinRead for CurrentOffset {
    type Args = ();

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        options: &binread::ReadOptions,
        args: Self::Args,
    ) -> binread::prelude::BinResult<Self> {
        Ok(Self(reader.stream_pos()?.try_into().expect("invalid offset")))
    }
}

impl AsRef<u32> for CurrentOffset {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}