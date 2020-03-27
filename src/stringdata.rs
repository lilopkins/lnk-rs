use byteorder::{ByteOrder, LE};

pub fn parse_string(data: &[u8]) -> (u16, String) {
    let chars = LE::read_u16(data) * 2;
    let mut s = String::new();
    for i in 0..chars {
        s.push(data[2 + i as usize] as char);
    }
    (chars, s)
}
