use eyre::{Result, WrapErr};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchResponse<T> {
    pub next: Option<String>,
    pub data: Vec<T>,
}

#[derive(Debug, Deserialize)]
pub struct Story {
    pub name: String,
    pub id: usize,
    pub app_url: String,
    pub story_type: String,
    pub deadline: Option<chrono::DateTime<chrono::Utc>>,
    pub planned_start_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct Epic {
    pub name: String,
    pub app_url: String,
    pub deadline: Option<chrono::DateTime<chrono::Utc>>,
    pub planned_start_date: Option<chrono::DateTime<chrono::Utc>>,
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

    pub async fn stories(&self, query: &str) -> Result<Vec<Story>> {
        self.search("stories", query).await
    }

    pub async fn epics(&self, query: &str) -> Result<Vec<Epic>> {
        self.search("epics", query).await
    }

    pub async fn search<T: serde::de::DeserializeOwned>(
        &self,
        kind: &str,
        query: &str,
    ) -> Result<Vec<T>> {
        let client = reqwest::Client::new();

        let mut out = Vec::with_capacity(16);
        let mut next = Some(String::new());

        while let Some(next_url) = next {
            let req = if next_url.is_empty() {
                client
                    .get(format!("https://api.app.shortcut.com/api/v3/search/{kind}"))
                    .query(&[("query", query)])
            } else {
                client.get(format!("https://api.app.shortcut.com{next_url}"))
            };

            let resp = req
                .header("Content-Type", "application/json")
                .header("Shortcut-Token", &self.token)
                .send()
                .await
                .wrap_err("could not make request to Shortcut's API")?;

            let mut search: SearchResponse<T> = resp
                .json()
                .await
                .wrap_err("could not read a search response from payload")?;

            next = search.next;

            out.append(&mut search.data)
        }

        Ok(out)
    }
}
