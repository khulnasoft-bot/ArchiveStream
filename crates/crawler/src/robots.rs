use robots_txt::Robots;
use reqwest::Client;
use tracing::warn;

pub struct RobotsChecker {
    client: Client,
}

impl RobotsChecker {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn can_crawl(&self, url_str: &str) -> bool {
        let url = match url::Url::parse(url_str) {
            Ok(u) => u,
            Err(_) => return false,
        };

        let robots_url = match url.join("/robots.txt") {
            Ok(u) => u,
            Err(_) => return true, // Fallback to allowing if robots.txt URL is invalid
        };

        match self.client.get(robots_url).send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    let text = resp.text().await.unwrap_or_default();
                    let robots = Robots::from_str(&text);
                    // Match against ArchiveStream user agent
                    robots.allowed(url_str, "ArchiveStream")
                } else {
                    true // If no robots.txt, assume allowed
                }
            }
            Err(e) => {
                warn!("Failed to fetch robots.txt for {}: {}", url_str, e);
                true // Fallback to allowed on network error
            }
        }
    }
}
