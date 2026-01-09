use chrono::{DateTime, Utc};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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

    /// Broadcast a search query to all active peers
    pub async fn broadcast_search(&self, query: &FederatedQuery) -> Vec<FederatedSearchResult> {
        let peers = self.get_active_peers();
        let client = reqwest::Client::new();
        let mut tasks = vec![];

        for peer in peers {
            let client = client.clone();
            let q = query.clone();
            tasks.push(tokio::spawn(async move {
                // Assuming peer endpoint is the base URL like "http://10.0.0.1:3000"
                let url = format!("{}/api/v1/search", peer.endpoint);

                match client
                    .get(&url)
                    .query(&[("q", &q.query)])
                    .timeout(std::time::Duration::from_secs(2))
                    .send()
                    .await
                {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            match resp.json::<Vec<serde_json::Value>>().await {
                                Ok(results) => Some(FederatedSearchResult {
                                    source_node_id: peer.id,
                                    results,
                                }),
                                Err(_) => None,
                            }
                        } else {
                            None
                        }
                    }
                    Err(_) => None,
                }
            }));
        }

        let mut all_results = vec![];
        for task in tasks {
            if let Ok(Some(res)) = task.await {
                all_results.push(res);
            }
        }
        all_results
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedQuery {
    pub query: String,
    pub max_instances: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedSearchResult {
    pub source_node_id: String,
    pub results: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestRequest {
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestResponse {
    pub snapshots: Vec<archive_common::Snapshot>,
}
