use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Snapshot {
    pub id: Uuid,
    pub url: String,
    pub timestamp: DateTime<Utc>,
    pub warc_file: String,
    pub offset: u64,
    pub length: u64,
    pub sha256: String,
    pub status_code: u16,
    pub content_type: String,
}

pub mod warc {
    // WARC related utilities will go here
}
