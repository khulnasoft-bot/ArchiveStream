use serde::{Deserialize, Serialize};
use std::sync::Arc;
use dashmap::DashMap;
use chrono::{DateTime, Utc};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    pub id: String,
    pub endpoint: String,
    pub last_seen: DateTime<Utc>,
    pub status: PeerStatus,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PeerStatus {
    Active,
    Unreachable,
    Banned,
}

pub struct PeerManager {
    // Map of NodeID -> Peer
    peers: Arc<DashMap<String, Peer>>,
    local_node_id: String,
}

impl PeerManager {
    pub fn new(local_node_id: String) -> Self {
        Self {
            peers: Arc::new(DashMap::new()),
            local_node_id,
        }
    }

    pub fn add_peer(&self, id: String, endpoint: String) {
        let peer = Peer {
            id: id.clone(),
            endpoint,
            last_seen: Utc::now(),
            status: PeerStatus::Active,
            capabilities: vec!["search".to_string(), "sync".to_string()],
        };
        self.peers.insert(id, peer);
    }

    pub fn get_active_peers(&self) -> Vec<Peer> {
        self.peers
            .iter()
            .filter(|p| p.status == PeerStatus::Active)
            .map(|p| p.value().clone())
            .collect()
    }

    pub fn update_last_seen(&self, id: &str) {
        if let Some(mut peer) = self.peers.get_mut(id) {
            peer.last_seen = Utc::now();
            peer.status = PeerStatus::Active;
        }
    }

    /// Retrieve the list of peers formatted for API response
    pub fn get_peers_json(&self) -> serde_json::Value {
        serde_json::json!({
            "local_node_id": self.local_node_id,
            "peers": self.get_active_peers()
        })
    }
}
