use axum::http::HeaderMap;
use base64::prelude::*;
use log::debug;
use reqwest::{Client, Response};

pub struct NRStatic {
    client: Client,
}

impl NRStatic {
    pub fn new() -> anyhow::Result<Self> {
        let username = std::env::var("NRSTATIC_USERNAME")?;
        let password = std::env::var("NRSTATIC_PASSWORD")?;

        let base64_auth = BASE64_STANDARD.encode(format!("{username}:{password}"));

        let mut headers = HeaderMap::new();
        headers.append(
            "Authorization",
            format!("Basic {}", base64_auth).parse().unwrap(),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self { client })
    }

    async fn fetch(self, url: String) -> anyhow::Result<Response> {
        let data = self.client.get(url).send().await?;
        Ok(data)
    }

    pub async fn fetch_full(self, url: String) -> anyhow::Result<Vec<u8>> {
        let response = self.fetch(url).await?;
        Ok(response.bytes().await?.to_vec())
    }

    pub async fn fetch_etag(self, url: String) -> anyhow::Result<String> {
        let response = self.fetch(url).await?;
        let etag = response
            .headers()
            .get("ETag")
            .unwrap()
            .to_str()?
            .to_string()
            .replace("\"\\", "")
            .replace("\"", "");
        debug!("etag: {}", etag);
        Ok(String::new())
    }
}
