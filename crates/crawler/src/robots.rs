use reqwest::Client;
use robotstxt::DefaultMatcher;
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
            Err(_) => return true,
        };

        match self.client.get(robots_url.clone()).send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    let text = resp.text().await.unwrap_or_default();
                    let mut matcher = DefaultMatcher::default();
                    matcher.one_agent_allowed_by_robots(&text, "ArchiveStream", url_str)
                } else {
                    true
                }
            }
            Err(e) => {
                warn!("Failed to fetch robots.txt for {}: {}", url_str, e);
                true
            }
        }
    }
}

impl Default for RobotsChecker {
    fn default() -> Self {
        Self::new()
    }
}
