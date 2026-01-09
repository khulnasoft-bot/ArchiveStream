use anyhow::Result;
use opensearch::{http::transport::Transport, indices::IndicesCreateParts, BulkParts, OpenSearch};
use serde_json::{json, Value};

pub struct SearchClient {
    client: OpenSearch,
}

impl SearchClient {
    pub fn new(url: &str) -> Result<Self> {
        let transport = Transport::single_node(url)?;
        let client = OpenSearch::new(transport);
        Ok(Self { client })
    }

    pub async fn ensure_index(&self) -> Result<()> {
        let index_name = "snapshots";
        let exists = self
            .client
            .indices()
            .exists(opensearch::indices::IndicesExistsParts::Index(&[
                index_name,
            ]))
            .send()
            .await?;

        if exists.status_code() == http::StatusCode::NOT_FOUND {
            self.client
                .indices()
                .create(IndicesCreateParts::Index(index_name))
                .body(json!({
                    "mappings": {
                        "properties": {
                            "snapshot_id": { "type": "keyword" },
                            "url": { "type": "text" },
                            "domain": { "type": "keyword" },
                            "timestamp": { "type": "date" },
                            "title": { "type": "text" },
                            "content": { "type": "text" },
                            "mime": { "type": "keyword" },
                        }
                    }
                }))
                .send()
                .await?;
        }

        Ok(())
    }

    pub async fn index_snapshots(&self, docs: Vec<Value>) -> Result<()> {
        if docs.is_empty() {
            return Ok(());
        }

        let mut body: Vec<opensearch::BulkOperation<Value>> = Vec::new();
        for doc in docs {
            body.push(opensearch::BulkOperation::index(doc).into());
        }

        self.client
            .bulk(BulkParts::Index("snapshots"))
            .body(body)
            .send()
            .await?;

        Ok(())
    }
}
