use archive_crawler::Crawler;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    info!("ArchiveStream Crawler Starting...");

    let crawler = Crawler::new();
    crawler.add_url("https://example.com".to_string()).await;
    
    crawler.run().await?;

    Ok(())
}
