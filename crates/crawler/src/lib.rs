pub mod fetcher;
pub mod parser;
pub mod robots;
pub mod warc;

use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, error};
use crate::fetcher::Fetcher;

pub struct Crawler {
    fetcher: Fetcher,
    queue: Arc<Mutex<VecDeque<String>>>,
}

impl Crawler {
    pub fn new() -> Self {
        Self {
            fetcher: Fetcher::new(),
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub async fn add_url(&self, url: String) {
        let mut queue = self.queue.lock().await;
        queue.push_back(url);
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        info!("Crawler loop started");
        loop {
            let url = {
                let mut queue = self.queue.lock().await;
                queue.pop_front()
            };

            if let Some(url) = url {
                info!("Crawling: {}", url);
                match self.fetcher.fetch(&url).await {
                    Ok(record) => {
                        info!("Fetched {} ({} bytes)", url, record.content.len());
                        // TODO: Save WARC to S3
                        // TODO: Save metadata to DB
                        
                        // Extract links for basic crawling (optional/limited)
                        if record.content_type.contains("html") {
                            let html = String::from_utf8_lossy(&record.content);
                            let links = parser::extract_links(&url, &html);
                            info!("Extracted {} links", links.len());
                            // For MVP, we might not want to crawl infinitely
                        }
                    }
                    Err(e) => {
                        error!("Failed to fetch {}: {}", url, e);
                    }
                }
            } else {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        }
    }
}
