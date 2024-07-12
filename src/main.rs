mod shortcut;

use clap::Parser;
use color_eyre::Result;

#[derive(Debug, Parser)]
struct Cli {
    /// The token to use to request information from Shortcut's REST API
    #[clap(env = "SHORTCUT_API_TOKEN")]
    shortcut_api_token: String,
}

impl Cli {
    async fn run(&self) -> Result<()> {
        let client = shortcut::Client::new(&self.shortcut_api_token);

        let stories = client.stories("owner:brnhx state:Ready").await?;
        println!("{stories:#?}");

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    color_eyre::install().expect("color_eyre to install handlers");

    let cli = Cli::parse();

    if let Err(err) = cli.run().await {
        println!("{err:?}");
        std::process::exit(1)
    }
}
