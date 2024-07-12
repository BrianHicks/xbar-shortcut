use clap::Parser;

#[derive(Debug, Parser)]
struct Opts {
    /// The token to use to request information from Shortcut's REST API
    shortcut_token: String,
}

fn main() {
    let opts = Opts::parse();

    println!("{opts:#?}")
}
