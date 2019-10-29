extern crate lnk;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 0 {
        eprintln!("You must specify some file(s) to read!");
    }

    for arg in &args[1..] {
        println!("{}: ", arg);
        let shortcut = lnk::ShellLink::open(arg).unwrap();
        println!("{:#?}", shortcut);
    }
}
