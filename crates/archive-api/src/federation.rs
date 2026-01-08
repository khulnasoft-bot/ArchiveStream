use axum::{
    extract::{State, Json},
    response::IntoResponse,
};
use std::sync::Arc;
use crate::AppState;
use archive_federation::Peer;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct HandshakeRequest {
    pub node_id: String,
    pub endpoint: String,
}

/// GET /api/v1/federation/peers
/// Returns the list of known active peers
pub async fn get_peers(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let peers_json = state.peer_manager.get_peers_json();
    Json(peers_json)
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
