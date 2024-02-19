use binread::{io::StreamPosition, BinRead};
use log::trace;

/// implements [`BinRead`] by reading the current cursor position
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
        let pos = reader.stream_pos()?;
        trace!("read offset at 0x{pos:016x}");
        Ok(Self(pos.try_into().expect("invalid offset")))
    }
}

impl AsRef<u32> for CurrentOffset {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}