use serde_json::json;

use crate::error::AppResult;
use crate::rpc::RpcClient;

pub fn run(client: &RpcClient) -> AppResult<()> {
    let address: String = client.call_typed("getnewaddress", json!([]))?;
    println!("{address}");
    Ok(())
}