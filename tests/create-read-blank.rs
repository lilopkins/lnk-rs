use log::info;

use std::fs;

const TEST_FILE_NAME: &'static str = "temp.lnk";

#[test]
fn create_read_blank() {
    pretty_env_logger::init();

    for is_unicode in &[false, true] {
        info!("Saving shortcut...");
        let mut shortcut = lnk::ShellLink::default();
        shortcut
            .header
            .link_flags
            .set(lnk::LinkFlags::IS_UNICODE, *is_unicode);
        shortcut.set_name(Some("Blank name".to_string()));
        shortcut
            .save(TEST_FILE_NAME)
            .expect("Failed to save shortcut!");

        info!("Reading shortcut...");
        let shortcut = lnk::ShellLink::open(TEST_FILE_NAME).unwrap();
        println!("{:#?}", shortcut);
        assert_eq!(shortcut.name(), &Some("Blank name".to_string()));
    }

    info!("Cleaning up...");
    fs::remove_file(TEST_FILE_NAME).expect("delete shortcut");
}
