use crate::error::{AppError, AppResult};

/// Holds everything needed to connect to a Bitcoin Core RPC endpoint.
#[derive(Debug, Clone)]
pub struct Config {
    pub rpc_url: String,
    pub rpc_user: String,
    pub rpc_password: String,
    pub wallet_name: Option<String>,
}

impl Config {
    /// Load config from environment variables (optionally populated by a .env file).
    ///
    /// Expected variables:
    ///   RPC_URL      e.g. http://127.0.0.1:18443
    ///   RPC_USER     e.g. polaruser
    ///   RPC_PASSWORD e.g. polarpass
    ///   WALLET_NAME  optional, e.g. "" or "my_wallet"
    pub fn from_env() -> AppResult<Self> {
        // Load .env file if present; ignore error if it's missing (env vars may be set another way).
        let _ = dotenvy::dotenv();

        let rpc_url = std::env::var("RPC_URL")
            .map_err(|_| AppError::Config("RPC_URL is not set (env var or .env file)".into()))?;

        let rpc_user = std::env::var("RPC_USER")
            .map_err(|_| AppError::Config("RPC_USER is not set (env var or .env file)".into()))?;

        let rpc_password = std::env::var("RPC_PASSWORD")
            .map_err(|_| AppError::Config("RPC_PASSWORD is not set (env var or .env file)".into()))?;

        let wallet_name = std::env::var("WALLET_NAME").ok().filter(|s| !s.is_empty());

        Ok(Config {
            rpc_url,
            rpc_user,
            rpc_password,
            wallet_name,
        })
    }

    /// Build the full RPC URL, including the wallet path if a wallet is configured.
    /// Bitcoin Core expects wallet-specific calls at /wallet/<name>.
    pub fn rpc_endpoint(&self) -> String {
        match &self.wallet_name {
            Some(name) => format!("{}/wallet/{}", self.rpc_url.trim_end_matches('/'), name),
            None => self.rpc_url.clone(),
        }
    }
}