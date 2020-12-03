use anyhow::Error;
use reqwest::{header::AUTHORIZATION, Client};
use serde::de::DeserializeOwned;

#[macro_use]
mod macros;
pub mod urls;
pub mod destiny2;
pub use destiny2::Destiny2;

pub struct BungieClient {
    api_key: String,
    oauth_token: Option<String>,
}

impl BungieClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            oauth_token: None,
        }
    }

    pub fn with_authentication_token(mut self, oauth_token: String) -> Self {
        self.oauth_token = Some(oauth_token);
        self
    }

    async fn send_request<T: DeserializeOwned>(
        &self,
        path: &str,
        body: Option<String>,
    ) -> Result<T, Error> {
        let client = Client::new();
        let path = [urls::API, path].join("/");

        let mut req = body
            .map_or_else(|| client.get(&path), |body| client.post(&path).body(body))
            .header("X-API-Key", &self.api_key);

        if let Some(ref oauth_token) = self.oauth_token {
            req = req.header(AUTHORIZATION, oauth_token);
        }

        let response = req.send().await?;
        Ok(response.json().await?)
    }
}
