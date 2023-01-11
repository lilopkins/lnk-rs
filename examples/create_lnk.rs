use std::io::Result;
use std::path::Path;

fn main() -> Result<()> {
    pretty_env_logger::init();

    let shortcut = lnk::ShellLink::new_simple(Path::new(r"C:\Windows\System32\notepad.exe"))?;
    shortcut.save("np.lnk").expect("Failed to save shortcut!");
    Ok(())
}
