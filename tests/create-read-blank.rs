use lnk::{encoding::WINDOWS_1252, StringEncoding};
use log::info;

use std::fs;

const TEST_FILE_NAME: &'static str = "temp.lnk";

#[test]
fn create_read_blank() {
    pretty_env_logger::init();

    for encoding in &[
        StringEncoding::Unicode,
        StringEncoding::CodePage(WINDOWS_1252),
    ] {
        info!("Saving shortcut...");
        let mut shortcut = lnk::ShellLink::default().with_encoding(encoding);
        shortcut.set_name(Some("Blank name".to_string()));
        shortcut
            .save(TEST_FILE_NAME)
            .expect("Failed to save shortcut!");

        info!("Reading shortcut...");

        let shortcut = lnk::ShellLink::open(TEST_FILE_NAME, encoding.encoding()).unwrap();
        //println!("{:#?}", shortcut);
        assert_eq!(
            shortcut.string_data().name_string(),
            &Some("Blank name".to_string())
        );
    }

    info!("Cleaning up...");
    fs::remove_file(TEST_FILE_NAME).expect("delete shortcut");
}
