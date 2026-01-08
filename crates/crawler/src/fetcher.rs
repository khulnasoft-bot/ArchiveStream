use reqwest::Client;
use crate::warc::WarcRecord;

pub struct Fetcher {
    client: Client,
}

impl Fetcher {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("ArchiveStream/0.1.0 (+https://github.com/KhulnaSoft/ArchiveStream)")
                .build()
                .unwrap(),
        }
    }

    pub async fn fetch(&self, url: &str) -> anyhow::Result<WarcRecord> {
        let response = self.client.get(url).send().await?;
        let status = response.status();
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("text/html")
            .to_string();
        
        let content = response.bytes().await?.to_vec();
        
        Ok(WarcRecord {
            url: url.to_string(),
            timestamp: chrono::Utc::now(),
            content,
            content_type,
        })
    }
}
