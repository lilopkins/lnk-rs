extern crate lnk;

fn main() {
    let shortcut = lnk::ShellLink::new();
    shortcut.save("demo.lnk").expect("Failed to save shortcut!");
}
