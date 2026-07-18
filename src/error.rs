use thiserror::Error;

/// All errors that can occur in this application.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Failed to connect to Bitcoin Core node at {url}: {source}")]
    Connection {
        url: String,
        #[source]
        source: reqwest::Error,
    },

    #[error("Authentication failed: check your RPC username/password")]
    Auth,

    #[error("Bitcoin Core RPC error (code {code}): {message}")]
    Rpc { code: i64, message: String },

    #[error("Invalid response from node: {0}")]
    InvalidResponse(String),

    #[error("Wallet error: {0}")]
    Wallet(String),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

pub type AppResult<T> = Result<T, AppError>;