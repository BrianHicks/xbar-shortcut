use clap::Parser;
use color_eyre::Result;

#[derive(Debug, Parser)]
struct Cli {
    /// The token to use to request information from Shortcut's REST API
    shortcut_token: String,
}

impl Cli {
    fn run(&self) -> Result<()> {
        color_eyre::eyre::bail!("TODO")
    }
}

fn main() {
    color_eyre::install().expect("color_eyre to install handlers");

    let cli = Cli::parse();

    if let Err(err) = cli.run() {
        println!("{err:?}");
        std::process::exit(1)
    }
}
