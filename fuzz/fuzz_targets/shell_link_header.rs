#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _shell_link_header = lnk::ShellLinkHeader::try_from(&data[0..0x4c]);
});
