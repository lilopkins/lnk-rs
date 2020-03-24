use std::fs;

const TEST_FILE_NAME: &'static str = "test.lnk";

#[test]
fn test_lnk() {
    let shortcut = lnk::ShellLink::open(TEST_FILE_NAME).unwrap();
    
}
