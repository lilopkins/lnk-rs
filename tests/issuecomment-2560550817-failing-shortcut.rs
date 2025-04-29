use lnk::{encoding::WINDOWS_1252, ShellLink};
use log::debug;

///https://github.com/lilopkins/lnk-rs/pull/21#issuecomment-2560550817

#[test]
fn tes_failing_shortcut() {
    let _ = pretty_env_logger::try_init();

    let shortcut = ShellLink::open("tests/data/iron-heart.exe - Shortcut.lnk", WINDOWS_1252).unwrap();
    debug!("{:#?}", shortcut);
}

#[test]
fn test_non_latin_shortcut() {
    let _ = pretty_env_logger::try_init();

    let shortcut = ShellLink::open("tests/data/iron-heart.exe - non-latin Shortcut.lnk", WINDOWS_1252).unwrap();
    debug!("{:#?}", shortcut);
}