#![no_main]

use libfuzzer_sys::fuzz_target;
use lnk::ShellLinkHeader;
use std::io::Cursor;
use binread::BinReaderExt;

fuzz_target!(|data: &[u8]| {
    let mut cursor = Cursor::new(data);
    match cursor.read_le::<ShellLinkHeader>() {
        Err(_) => (),
        Ok(header) => println!("fuzzer found as valid header of size {}", header.header_size())
    }
});
