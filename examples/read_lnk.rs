use std::env;

fn main() {
    pretty_env_logger::init();
    
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        eprintln!("You must specify some file(s) to read!");
    }

    for arg in &args[1..] {
        println!("{}: ", arg);
        let shortcut = lnk::ShellLink::open(arg).unwrap();
        println!("{:#?}", shortcut);
    }
}
