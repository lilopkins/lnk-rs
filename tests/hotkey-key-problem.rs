use lnk::{encoding::WINDOWS_1252, HotkeyKey, HotkeyModifiers, ShellLink};

///https://github.com/lilopkins/lnk-rs/pull/27

#[test]
fn tes_failing_hotkey_key_shortcut() {
    let _ = pretty_env_logger::try_init();

    let shortcut = ShellLink::open("tests/data/hotkey-problem.lnk", WINDOWS_1252).unwrap();
    assert_eq!(*shortcut.header().hotkey().key(), HotkeyKey::NoKeyAssigned);
    assert_eq!(
        *shortcut.header().hotkey().modifiers(),
        HotkeyModifiers::NO_MODIFIER
    );
}

#[test]
fn tes_no_failing_hotkey_key_shortcut() {
    let _ = pretty_env_logger::try_init();

    let shortcut = ShellLink::open("tests/data/hotkey-no-problem.lnk", WINDOWS_1252).unwrap();
    assert_eq!(*shortcut.header().hotkey().key(), HotkeyKey::Key1);
    assert_eq!(
        *shortcut.header().hotkey().modifiers(),
        HotkeyModifiers::HOTKEYF_ALT | HotkeyModifiers::HOTKEYF_CONTROL
    );
}
