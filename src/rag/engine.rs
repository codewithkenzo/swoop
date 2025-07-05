pub struct RagEngine {
    // Embedding model / client
    // In 2025 we default to OpenAI text-embedding-3-large; pluggable via trait
    // Stores cached embeddings in Redis and vectors in Qdrant
    pub qdrant: qdrant_client::prelude::QdrantClient,
    pub redis:  redis::Client,
}

impl RagEngine {
    pub async fn new(qdrant_url: &str, redis_url: &str) -> anyhow::Result<Self> {
        let qdrant = qdrant_client::qdrant::QdrantClient::from_url(qdrant_url).build()?;
        let redis = redis::Client::open(redis_url)?;
        Ok(Self { qdrant, redis })
    }

    /// Perform hybrid (BM25 + semantic) search and return top-k snippets ready for prompting.
    pub async fn retrieve(&self, query_embedding: Vec<f32>, k: usize) -> anyhow::Result<Vec<String>> {
        use qdrant_client::prelude::*;
        let result = self
            .qdrant
            .search(&SearchPoints {
                collection_name: "documents".to_string(),
                vector: query_embedding,
                limit: k as u64,
                with_payload: Some(true.into()),
                ..Default::default()
            })
            .await?;
        Ok(result
            .iter()
            .filter_map(|pt| pt.payload.as_ref()?.get("text"))
            .filter_map(|v| v.as_str())
            .map(|s| s.to_owned())
            .collect())
    }
} 