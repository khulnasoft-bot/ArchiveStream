use axum::{
    extract::{State, Json, Query, Path},
    response::IntoResponse,
    http::StatusCode,
};
use std::sync::Arc;
use crate::AppState;
// use archive_federation::Peer;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct HandshakeRequest {
    pub node_id: String,
    pub endpoint: String,
}

#[derive(Deserialize)]
pub struct FederatedSearchRequest {
    pub q: String,
}

/// GET /api/v1/federation/peers
/// Returns the list of known active peers
pub async fn get_peers(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let peers_json = state.peer_manager.get_peers_json();
    Json(peers_json)
}

/// GET /api/v1/federation/search
/// Performs a search across local and federated instances
pub async fn search_federated(
    State(state): State<Arc<AppState>>,
    Query(params): Query<FederatedSearchRequest>,
) -> impl IntoResponse {
    // 1. Local Search
    // Note: search_service.search returns Vec<SearchResult> or similar
    let local_results = state.search_service.search(&params.q).await.unwrap_or_default();

    // 2. Federated Search
    let fed_query = archive_federation::FederatedQuery {
        query: params.q.clone(),
        max_instances: None,
    };
    
    let peer_results = state.peer_manager.broadcast_search(&fed_query).await;

    // 3. Return Combined
    Json(serde_json::json!({
        "local": local_results,
        "federated": peer_results
    }))
}

/// POST /api/v1/federation/handshake
/// Accepts a handshake from another node
pub async fn handle_handshake(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<HandshakeRequest>,
) -> impl IntoResponse {
    // In a real implementation, we would verify the node_id and challenge
    // For now, we trust the handshake and add the peer
    state.peer_manager.add_peer(payload.node_id.clone(), payload.endpoint.clone());
    
    tracing::info!("Accepted handshake from peer: {}", payload.node_id);

    Json(serde_json::json!({
        "status": "accepted",
        "local_node_id": state.config.node_id // We need to add this to AppConfig
    }))
}

/// GET /api/v1/federation/manifest
/// Returns available snapshots for sync
pub async fn get_manifest(
    State(state): State<Arc<AppState>>,
    Query(params): Query<archive_federation::ManifestRequest>,
) -> impl IntoResponse {
    // Default to last 24 hours if not specified
    let from_ts = params.from.unwrap_or_else(|| chrono::Utc::now() - chrono::Duration::hours(24));
    let to_ts = params.to.unwrap_or_else(chrono::Utc::now);
    let limit = params.limit.unwrap_or(100) as i64;

    let result = sqlx::query_as::<_, archive_common::Snapshot>(
        r#"
        SELECT id, url, timestamp, warc_file, offset, length, sha256, status_code, content_type, payload_hash
        FROM snapshots
        WHERE timestamp >= $1 AND timestamp <= $2
        ORDER BY timestamp DESC
        LIMIT $3
        "#
    )
    .bind(from_ts)
    .bind(to_ts)
    .bind(limit)
    .fetch_all(&state.pool)
    .await;

    match result {
        Ok(snapshots) => Json(archive_federation::ManifestResponse { snapshots }).into_response(),
        Err(e) => {
            tracing::error!("Manifest error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Manifest failed").into_response()
        }
    }
}

/// GET /api/v1/snapshot/:id/download
/// Streams the raw WARC record
pub async fn download_snapshot(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let snapshot = match sqlx::query_as::<_, archive_common::Snapshot>(
        "SELECT id, url, timestamp, warc_file, offset, length, sha256, status_code, content_type, payload_hash FROM snapshots WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await {
        Ok(Some(s)) => s,
        Ok(None) => return (StatusCode::NOT_FOUND, "Snapshot not found").into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response(),
    };

    let data = match state.warc_reader.read_record(&snapshot.warc_file, snapshot.offset, snapshot.length).await {
        Ok(d) => d,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Read error").into_response(),
    };

    axum::response::Response::builder()
        .header("Content-Type", "application/warc-record")
        .header("Content-Disposition", format!("attachment; filename=\"{}.warc\"", id))
        .body(axum::body::Body::from(data))
        .unwrap()
        .into_response()
}

// --- Sync Worker ---

pub struct SyncWorker {
    state: Arc<AppState>,
}

impl SyncWorker {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub async fn run_loop(&self) {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(60)); // Poll every minute
        loop {
            interval.tick().await;
            if let Err(e) = self.sync_cycle().await {
                tracing::error!("Sync cycle error: {}", e);
            }
        }
    }

    async fn sync_cycle(&self) -> anyhow::Result<()> {
        let peers = self.state.peer_manager.get_active_peers();
        if peers.is_empty() {
            return Ok(());
        }

        let client = reqwest::Client::new();
        
        for peer in peers {
            // 1. Get Manifest
            let manifest_url = format!("{}/api/v1/federation/manifest?limit=50", peer.endpoint); // simplified
            let resp = client.get(&manifest_url).send().await?;
            if !resp.status().is_success() { continue; }
            
            let manifest: archive_federation::ManifestResponse = resp.json().await?;

            // 2. Process Snapshots
            for remote_snap in manifest.snapshots {
                // Check if we have it
                let exists = sqlx::query("SELECT 1 FROM snapshots WHERE sha256 = $1")
                    .bind(remote_snap.sha256.clone())
                    .fetch_optional(&self.state.pool)
                    .await?
                    .is_some();
                
                if !exists {
                    tracing::info!("Syncing missing snapshot {} from peer {}", remote_snap.id, peer.id);
                    self.download_and_save(&client, &peer, &remote_snap).await?;
                }
            }
        }
        Ok(())
    }

    async fn download_and_save(&self, client: &reqwest::Client, peer: &archive_federation::Peer, snap: &archive_common::Snapshot) -> anyhow::Result<()> {
        let url = format!("{}/api/v1/snapshot/{}/download", peer.endpoint, snap.id);
        let resp = client.get(&url).send().await?;
        let bytes = resp.bytes().await?;

        // Write to local WARC
        // For MVP, we just append to a dedicated sync.warc file
        
        // Improve: reconstruct WarcRecord or write raw bytes if WarcWriter supported raw write
        // Since WarcWriter expects WarcRecord, we need to parse the raw bytes or extend WarcWriter.
        // For now, let's assume we implement a write_raw method or parse it.
        // Actually, easier: The API returns the raw record (headers + body). 
        // We can just append it to the file directly.
        
        let path = "data/archive/sync.warc";
        if let Some(parent) = std::path::Path::new(path).parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        
        let offset = file.metadata()?.len();
        std::io::Write::write_all(&mut file, &bytes)?;
        let length = bytes.len() as u64;

        // Save to DB
        sqlx::query(
            "INSERT INTO snapshots (id, url, timestamp, warc_file, offset, length, sha256, status_code, content_type, payload_hash) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"
        )
        .bind(uuid::Uuid::new_v4())
        .bind(&snap.url)
        .bind(snap.timestamp)
        .bind("sync.warc")
        .bind(offset as i64)
        .bind(length as i64)
        .bind(&snap.sha256)
        .bind(snap.status_code as i32)
        .bind(&snap.content_type)
        .bind(&snap.payload_hash)
        .execute(&self.state.pool)
        .await?;

        Ok(())
    }
}
