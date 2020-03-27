use std::fs;

const TEST_FILE_NAME: &'static str = "temp.lnk";

#[test]
fn create_read_blank() {
    {
        let shortcut = lnk::ShellLink::default();
        shortcut.save(TEST_FILE_NAME).expect("Failed to save shortcut!");
    }

    {
        let shortcut = lnk::ShellLink::open(TEST_FILE_NAME).unwrap();
        println!("{:#?}", shortcut);
    }

    fs::remove_file(TEST_FILE_NAME).expect("delete shortcut");
}
