use eyre::{Result, WrapErr};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchResponse<T> {
    pub next: Option<String>,
    pub total: usize,
    pub data: Vec<T>,
}

#[derive(Debug, Deserialize)]
pub struct Story {
    pub name: String,
    pub app_url: String,
    pub story_type: String,
    pub deadline: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Epic {
    pub name: String,
    pub app_url: String,
    pub deadline: String,
    pub state: String,
}

pub struct Client {
    token: String,
}

impl Client {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_owned(),
        }
    }

    pub async fn stories(&self) -> Result<Vec<Story>> {
        let client = reqwest::Client::new();

        let resp = client.get(
            "https://api.app.shortcut.com/api/v3/search/stories?query=owner:brnhx%20state:Ready",
        )
        .header("Content-Type", "application/json")
        .header("Shortcut-Token", &self.token)
        .send().await
        .wrap_err("could not make request to Shortcut's API")?;

        let search: SearchResponse<Story> = resp
            .json()
            .await
            .wrap_err("could not read a search response from payload")?;

        return Ok(search.data);
    }
}
