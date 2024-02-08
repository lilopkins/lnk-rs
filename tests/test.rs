const TEST_FILE_NAME: &str = "tests/test.lnk";
const TEST_BLANK_FILE_NAME: &str = "tests/blank.txt";

use chrono::NaiveDate;
use lnk::*;
#[allow(unused)]
use log::{debug, error, info, trace, warn};

#[test]
fn test_lnk_header() {
    let _ = pretty_env_logger::try_init();

    let shortcut = ShellLink::open(TEST_FILE_NAME).unwrap();
    debug!("{:#?}", shortcut);

    assert_eq!(
        *shortcut.header().link_flags(),
        LinkFlags::HAS_LINK_TARGET_ID_LIST
            | LinkFlags::HAS_LINK_INFO
            | LinkFlags::HAS_RELATIVE_PATH
            | LinkFlags::HAS_WORKING_DIR
            | LinkFlags::IS_UNICODE
            | LinkFlags::ENABLE_TARGET_METADATA,
        "Link flags should be parsed correctly"
    );

    assert_eq!(
        *shortcut.header().file_attributes(),
        FileAttributeFlags::FILE_ATTRIBUTE_ARCHIVE,
        "File attributes should be parsed correctly"
    );

    assert_eq!(
        shortcut.header().creation_time().datetime().date(),
        NaiveDate::from_ymd_opt(2008, 9, 12).unwrap(),
        "Creation time should be parsed correctly"
    );
    assert_eq!(
        shortcut.header().access_time().datetime().date(),
        NaiveDate::from_ymd_opt(2008, 9, 12).unwrap(),
        "Access time should be parsed correctly"
    );
    assert_eq!(
        shortcut.header().write_time().datetime().date(),
        NaiveDate::from_ymd_opt(2008, 9, 12).unwrap(),
        "Write time should be parsed correctly"
    );

    assert_eq!(
        *shortcut.header().file_size(),
        0x00,
        "File size should be parsed correctly"
    );
    assert_eq!(
        *shortcut.header().icon_index(),
        0x00,
        "Icon index should be parsed correctly"
    );
    assert_eq!(
        *shortcut.header().show_command(),
        ShowCommand::ShowNormal,
        "Show command should be parsed correctly"
    );
    assert_eq!(*shortcut.header().hotkey().key(), HotkeyKey::NoKeyAssigned);
    assert_eq!(
        *shortcut.header().hotkey().modifiers(),
        HotkeyModifiers::NO_MODIFIER
    );

    assert_eq!(shortcut.name(), &None);
    assert_eq!(shortcut.relative_path(), &Some(r".\a.txt".to_string()));
    assert_eq!(shortcut.working_dir(), &Some(r"C:\test".to_string()));
}

#[test]
fn test_no_panic_reading_other_filetypes() {
    let _ = pretty_env_logger::try_init();

    let res = ShellLink::open(TEST_BLANK_FILE_NAME);
    // Shouldn't have panicked by now!
    assert!(res.is_err());
}
