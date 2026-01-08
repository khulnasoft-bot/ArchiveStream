use chrono::Utc;
use uuid::Uuid;
use std::io::Write;
use std::fs::OpenOptions;

pub struct WarcRecord {
    pub url: String,
    pub timestamp: chrono::DateTime<Utc>,
    pub content: Vec<u8>,
    pub content_type: String,
    pub payload_digest: String,
}

impl WarcRecord {
    pub fn to_warc_bytes(&self) -> Vec<u8> {
        let record_id = Uuid::new_v4();
        let timestamp = self.timestamp.format("%Y-%m-%dT%H:%M:%SZ").to_string();
        
        let header = format!(
            "WARC/1.0\r\n\
            WARC-Type: response\r\n\
            WARC-Record-ID: <urn:uuid:{}>\r\n\
            WARC-Date: {}\r\n\
            WARC-Target-URI: {}\r\n\
            WARC-Payload-Digest: sha256:{}\r\n\
            Content-Type: application/http; msgtype=response\r\n\
            Content-Length: {}\r\n\
            \r\n",
            record_id,
            timestamp,
            self.url,
            self.payload_digest,
            self.content.len()
        );

        let mut bytes = header.into_bytes();
        bytes.extend_from_slice(&self.content);
        bytes.extend_from_slice(b"\r\n\r\n");
        bytes
    }
}

pub struct WarcWriter {
    file_path: String,
}

impl WarcWriter {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }

    /// Appends a record to the WARC file and returns (offset, length)
    pub fn write_record(&mut self, record: &WarcRecord) -> std::io::Result<(u64, u64)> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)?;

        let pos = file.metadata()?.len();
        let bytes = record.to_warc_bytes();
        file.write_all(&bytes)?;
        
        Ok((pos, bytes.len() as u64))
    }
}
