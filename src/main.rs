mod shortcut;

use clap::Parser;
use color_eyre::Result;
use slugify::slugify;

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

        let mut headline = String::with_capacity(16);
        let mut lines = Vec::with_capacity(32);

        lines.push("Stories | size=18".into());

        for story_state in &self.story_state {
            lines.push(format!("{story_state} | size=14"));

            let stories = client
                .stories(&format!(
                    "owner:{0} state:\"{story_state}\" -is:archived",
                    self.for_user
                ))
                .await?;

            for story in stories {
                let mut line = String::with_capacity(64);
                line.push_str(&story.name);
                line.push_str(" (");
                line.push_str(&story.story_type);
                line.push(')');

                if let Some(days) = days_remaining(story.planned_start_date, story.deadline) {
                    if days <= 1 {
                        headline.push_str("🟠");
                        line.push_str(" 🟠");
                    } else if days <= 3 {
                        headline.push('🟡');
                        line.push_str(" 🟡");
                    } else {
                        headline.push_str("🟢");
                        line.push_str(" 🟢");
                    }
                } else {
                    headline.push_str("⚪️");
                    line.push_str(" ⚪️");
                }

                line.push_str(" | href=");
                line.push_str(&story.app_url);

                lines.push(line);

                if let Some(deadline) = story.deadline {
                    lines.push(format!("-- due {}", deadline.format("%A, %B %-e")))
                }

                lines.push(format!(
                    "-- Copy URL | shell=bash param1=-c param2=\"printf '%s' '{}' | pbcopy\"",
                    story.app_url
                ));

                let branch_name = format!(
                    "{}/sc-{}/{}",
                    self.for_user,
                    story.id,
                    slugify!(&story.name, max_length = 40)
                );
                lines.push(format!(
                    "-- {branch_name} | shell=bash param1=-c param2=\"printf '%s' '{branch_name}' | pbcopy\""
                ))
            }
        }

        lines.push("Epics | size=18".into());

        for epic_state in &self.epic_state {
            let epics = client
                .epics(&format!("owner:{} state:\"{epic_state}\"", self.for_user))
                .await?;

            for epic in epics {
                let mut line = String::with_capacity(64);
                line.push_str(&epic.name);

                if let Some(days) = days_remaining(epic.planned_start_date, epic.deadline) {
                    if days <= 1 {
                        headline.push_str("⚠️");
                        line.push_str(" ⚠️");
                    } else if days <= 7 {
                        headline.push('🔜');
                        line.push_str(" 🔜");
                    }
                }

                line.push_str(" | href=");
                line.push_str(&epic.app_url);

                lines.push(line);

                if let Some(deadline) = epic.deadline {
                    lines.push(format!("-- due {}", deadline.format("%A, %B %-e")))
                }

                lines.push(format!(
                    "-- Copy URL | shell=bash param1=-c param2=\"printf '%s' '{}' | pbcopy\"",
                    epic.app_url
                ))
            }
        }

        if headline.is_empty() {
            println!("Shortcut");
        } else {
            println!("{}", headline);
        }

        println!("---\n{}", lines.join("\n"));

        Ok(())
    }
}

fn days_remaining(
    planned_start_date: Option<chrono::DateTime<chrono::Utc>>,
    deadline: Option<chrono::DateTime<chrono::Utc>>,
) -> Option<i64> {
    let now = chrono::Utc::now();

    if let Some(date) = planned_start_date {
        if date > now {
            return None;
        }
    }

    deadline.map(|date| (date - now).num_days())
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
