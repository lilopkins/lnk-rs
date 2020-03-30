use byteorder::{ByteOrder, LE, WriteBytesExt};
use log::debug;

pub fn parse_string(data: &[u8]) -> (u16, String) {
    let chars = LE::read_u16(data) * 2;
    let mut s = String::new();
    for i in 0..chars {
        s.push(data[2 + i as usize] as char);
    }
    debug!("Parsed string: {:?}", s);
    (chars, s)
}

pub fn to_data<S: Into<String>>(str_data: S) -> Vec<u8> {
    let s = str_data.into();
    let mut data = Vec::new();
    let chs = s.as_bytes();
    data.write_u16::<LE>(chs.len() as u16 + 1).unwrap();
    for c in chs {
        data.write_u16::<LE>(*c as u16).unwrap();
    }
    // Null terminator
    data.write_u8(0).unwrap();
    data.write_u8(0).unwrap();
    
    data
}
