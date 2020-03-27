const TEST_FILE_NAME: &'static str = "tests/test.lnk";

#[allow(unused)]
use log::{trace, debug, info, warn, error};
use lnk::*;

#[test]
fn test_lnk_header() {
    pretty_env_logger::init();

    let shortcut = ShellLink::open(TEST_FILE_NAME).unwrap();
    debug!("{:#?}", shortcut);

    assert_eq!(*shortcut.header().link_flags(),
          LinkFlags::HAS_LINK_TARGET_ID_LIST
        | LinkFlags::HAS_LINK_INFO
        | LinkFlags::HAS_RELATIVE_PATH
        | LinkFlags::HAS_WORKING_DIR
        | LinkFlags::IS_UNICODE
        | LinkFlags::ENABLE_TARGET_METADATA, "Link flags should be parsed correctly");

    assert_eq!(*shortcut.header().file_attributes(),
        FileAttributeFlags::FILE_ATTRIBUTE_ARCHIVE, "File attributes should be parsed correctly");

    assert_eq!(shortcut.header().creation_time(), 0x01c91515f2eee9d0, "Creation time should be parsed correctly");
    assert_eq!(shortcut.header().access_time(), 0x01c91515f2eee9d0, "Access time should be parsed correctly");
    assert_eq!(shortcut.header().write_time(), 0x01c91515f2eee9d0, "Write time should be parsed correctly");

    assert_eq!(shortcut.header().file_size(), 0x00, "File size should be parsed correctly");
    assert_eq!(shortcut.header().icon_index(), 0x00, "Icon index should be parsed correctly");
    assert_eq!(*shortcut.header().show_command(), ShowCommand::ShowNormal, "Show command should be parsed correctly");
    assert_eq!(*shortcut.header().hotkey().key(), HotkeyKey::NoKeyAssigned);
    assert_eq!(*shortcut.header().hotkey().modifiers(), HotkeyModifiers::NO_MODIFIER);
}
