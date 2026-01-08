use axum::{
    extract::{Query, Path},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use archive_common::Snapshot;
use uuid::Uuid;
use chrono::Utc;

#[derive(Deserialize)]
struct SearchQuery {
    url: String,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    data: T,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "archive_api=info,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(|| async { "ArchiveStream API v0.1.0" }))
        .route("/health", get(|| async { Json(serde_json::json!({"status": "ok"})) }))
        .route("/snapshots", get(get_snapshots))
        .route("/snapshot/:id", get(get_snapshot))
        .route("/crawl", post(start_crawl));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    tracing::info!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_snapshots(Query(params): Query<SearchQuery>) -> Json<ApiResponse<Vec<Snapshot>>> {
    tracing::info!("Searching snapshots for: {}", params.url);
    // Mock data for now
    let snapshots = vec![
        Snapshot {
            id: Uuid::new_v4(),
            url: params.url.clone(),
            timestamp: Utc::now(),
            warc_file: "example.warc.gz".to_string(),
            offset: 0,
            length: 1024,
            sha256: "fake-sha".to_string(),
            status_code: 200,
            content_type: "text/html".to_string(),
        }
    ];
    Json(ApiResponse { data: snapshots })
}

async fn get_snapshot(Path(id): Path<Uuid>) -> Json<ApiResponse<Snapshot>> {
    tracing::info!("Getting snapshot: {}", id);
    let snapshot = Snapshot {
        id,
        url: "https://example.com".to_string(),
        timestamp: Utc::now(),
        warc_file: "example.warc.gz".to_string(),
        offset: 0,
        length: 1024,
        sha256: "fake-sha".to_string(),
        status_code: 200,
        content_type: "text/html".to_string(),
    };
    Json(ApiResponse { data: snapshot })
}

async fn start_crawl(Json(payload): Json<SearchQuery>) -> Json<serde_json::Value> {
    tracing::info!("Starting crawl for: {}", payload.url);
    Json(serde_json::json!({"status": "queued", "url": payload.url}))
}
