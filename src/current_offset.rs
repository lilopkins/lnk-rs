use binrw::BinRead;
use log::trace;

/// implements [`BinRead`] by reading the current cursor position
/// and storing it as `u32`
#[derive(Clone, Debug)]
pub struct CurrentOffset(u32);

impl BinRead for CurrentOffset {
    type Args<'a> = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let pos = reader.stream_position()?;
        trace!("read offset at 0x{pos:016x}");
        Ok(Self(pos.try_into().expect("invalid offset")))
    }
}

impl AsRef<u32> for CurrentOffset {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}
