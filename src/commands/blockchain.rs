use serde::Deserialize;
use serde_json::json;

use crate::error::AppResult;
use crate::rpc::RpcClient;

#[derive(Deserialize, Debug)]
pub struct BlockchainInfo {
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
    pub difficulty: f64,
    #[serde(rename = "verificationprogress")]
    pub verification_progress: f64,
}

pub fn run(client: &RpcClient) -> AppResult<()> {
    let info: BlockchainInfo = client.call_typed("getblockchaininfo", json!([]))?;

    println!("Chain:                {}", info.chain);
    println!("Blocks:               {}", info.blocks);
    println!("Headers:              {}", info.headers);
    println!("Difficulty:           {}", info.difficulty);
    println!("Verification progress:{:.4}%", info.verification_progress * 100.0);

    Ok(())
}