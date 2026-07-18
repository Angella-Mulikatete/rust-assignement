use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::Config;
use crate::error::{AppError, AppResult};

#[derive(Serialize)]
struct RpcRequest<'a> {
    jsonrpc: &'a str,
    id: &'a str,
    method: &'a str,
    params: Value,
}

#[derive(Deserialize)]
struct RpcResponse {
    result: Option<Value>,
    error: Option<RpcErrorBody>,
}

#[derive(Deserialize)]
struct RpcErrorBody {
    code: i64,
    message: String,
}

/// A client for talking to a Bitcoin Core node via JSON-RPC.
pub struct RpcClient {
    http: reqwest::blocking::Client,
    config: Config,
}

impl RpcClient {
    pub fn new(config: Config) -> Self {
        Self {
            http: reqwest::blocking::Client::new(),
            config,
        }
    }

    /// Call an arbitrary RPC method with the given params, returning the raw JSON result.
    pub fn call(&self, method: &str, params: Value) -> AppResult<Value> {
        let endpoint = self.config.rpc_endpoint();

        let body = RpcRequest {
            jsonrpc: "1.0",
            id: "rust-bitcoin-cli",
            method,
            params,
        };

        let response = self
            .http
            .post(&endpoint)
            .basic_auth(&self.config.rpc_user, Some(&self.config.rpc_password))
            .json(&body)
            .send()
            .map_err(|e| AppError::Connection {
                url: endpoint.clone(),
                source: e,
            })?;

        let status = response.status();

        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(AppError::Auth);
        }

        // Bitcoin Core returns error details in the JSON body even for some 4xx/5xx statuses,
        // so we try to parse the body regardless of status code.
        let text = response
            .text()
            .map_err(|e| AppError::Connection { url: endpoint.clone(), source: e })?;

        let parsed: RpcResponse = serde_json::from_str(&text)
            .map_err(|_| AppError::InvalidResponse(format!("Unexpected response body: {text}")))?;

        if let Some(err) = parsed.error {
            // Bitcoin Core uses -18 for "wallet not found" style errors.
            if err.code == -18 || err.message.to_lowercase().contains("no wallet") {
                return Err(AppError::Wallet(err.message));
            }
            return Err(AppError::Rpc {
                code: err.code,
                message: err.message,
            });
        }

        parsed
            .result
            .ok_or_else(|| AppError::InvalidResponse("Response had no result and no error".into()))
    }

    /// Convenience: call an RPC method and deserialize the result into a typed struct.
    pub fn call_typed<T: for<'de> Deserialize<'de>>(
        &self,
        method: &str,
        params: Value,
    ) -> AppResult<T> {
        let result = self.call(method, params)?;
        serde_json::from_value(result).map_err(AppError::Serde)
    }
}