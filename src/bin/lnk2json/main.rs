use clap::{Parser, ValueHint};
use clio::Input;
use lnk::ShellLink;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

#[derive(Parser)]
#[clap(name="lnk2json", author, version, long_about = None)]
struct Cli {
    #[clap(value_hint=ValueHint::FilePath, help="path to lnk file")]
    pub(crate) input_file: Input,

    /// pretty print JSON output
    #[clap(short('P'), long("pretty"))]
    pub(crate) pretty: bool,

    #[clap(flatten)]
    pub(crate) verbose: clap_verbosity_flag::Verbosity,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let _ = TermLogger::init(
        cli.verbose.log_level_filter(),
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto);
    
    if ! cli.input_file.path().exists() {
        anyhow::bail!("the file you specified does not exist");
    }
    if ! cli.input_file.path().is_file() {
        anyhow::bail!("you did not specify a file");
    }

    let shell_link = ShellLink::open(cli.input_file.path().path())?;

    if cli.pretty {
        println!("{}", serde_json::to_string_pretty(&shell_link)?);
    } else {
        println!("{}", serde_json::to_string(&shell_link)?);        
    }
    Ok(())
}