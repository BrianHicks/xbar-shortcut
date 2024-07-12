mod shortcut;

use clap::Parser;
use color_eyre::Result;

#[derive(Debug, Parser)]
struct Cli {
    /// The token to use to request information from Shortcut's REST API
    #[clap(env = "SHORTCUT_API_TOKEN")]
    shortcut_api_token: String,

    /// Your name in Shortcut
    #[clap(long)]
    for_user: String,

    /// Story states you care about
    #[clap(long)]
    story_state: Vec<String>,

    /// Epic states you care about
    #[clap(long)]
    epic_state: Vec<String>,
}

impl Cli {
    async fn run(&self) -> Result<()> {
        let client = shortcut::Client::new(&self.shortcut_api_token);

        let now = chrono::Utc::now();

        for story_state in &self.story_state {
            let stories = client
                .stories(&format!("owner:{0} state:\"{story_state}\"", self.for_user))
                .await?;
            println!("{stories:?}");
        }

        for epic_state in &self.epic_state {
            let epics = client
                .epics(&format!("owner:{} state:\"{epic_state}\"", self.for_user))
                .await?;
            println!("{epics:#?}");

            for epic in epics {
                if let Some(deadline) = epic.deadline {
                    println!("{deadline} {0}", (deadline - now).num_days())
                }
            }
        }

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
