//! Error types for SurrealX

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Function error: {0}")]
    Function(String),

    #[error("Event error: {0}")]
    Event(String),

    #[error("Cache error: {0}")]
    Cache(String),

    #[error("Server error: {0}")]
    Server(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[cfg(feature = "redis-cache")]
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
