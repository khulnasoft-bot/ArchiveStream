use chrono::Utc;
use uuid::Uuid;

pub struct WarcRecord {
    pub url: String,
    pub timestamp: chrono::DateTime<Utc>,
    pub content: Vec<u8>,
    pub content_type: String,
}

impl WarcRecord {
    pub fn to_warc_string(&self) -> String {
        let record_id = Uuid::new_v4();
        let timestamp = self.timestamp.format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        format!(
            "WARC/1.0\r\n\
            WARC-Type: response\r\n\
            WARC-Record-ID: <urn:uuid:{}>\r\n\
            WARC-Date: {}\r\n\
            WARC-Target-URI: {}\r\n\
            Content-Type: application/http; msgtype=response\r\n\
            Content-Length: {}\r\n\
            \r\n",
            record_id,
            timestamp,
            self.url,
            self.content.len()
        )
    }
}
