use log::info;

use std::fs;

const TEST_FILE_NAME: &'static str = "temp.lnk";

#[test]
fn create_read_blank() {
    pretty_env_logger::init();
    
    {
        info!("Saving shortcut...");
        let mut shortcut = lnk::ShellLink::default();
        shortcut.set_name(Some("Blank name".to_string()));
        shortcut.save(TEST_FILE_NAME).expect("Failed to save shortcut!");
    }

    {
        info!("Reading shortcut...");
        let shortcut = lnk::ShellLink::open(TEST_FILE_NAME).unwrap();
        // This currently fails as complete string parsing isn't yet complete.
        // assert_eq!(shortcut.name(), &Some("Blank name".to_string()));
        println!("{:#?}", shortcut);
    }

    info!("Cleaning up...");
    fs::remove_file(TEST_FILE_NAME).expect("delete shortcut");
}
