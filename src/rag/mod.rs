//! Retrieval-Augmented Generation engine (feature `semantic-rag`).
//! Currently provides `RagEngine` backed by Qdrant + Redis.

#[cfg(feature = "semantic-rag")]
pub mod engine;

#[cfg(feature = "semantic-rag")]
pub use engine::RagEngine; 