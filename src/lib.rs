use anyhow::{Context, Error, anyhow};
use surf::http::headers::AUTHORIZATION;
use surf::middleware::Redirect;
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
        let client = surf::client();
        let path = [urls::API, path].join("/");

        let mut req = body
            .map_or_else(|| client.get(&path), |body| client.post(&path).body(body))
            .header("X-API-Key", &self.api_key);

        if let Some(ref oauth_token) = self.oauth_token {
            req = req.header(AUTHORIZATION, oauth_token);
        }

        req.recv_json().await
            .map_err(|e| anyhow!(e))
            .with_context(|| "failed to receive json")
    }
}
