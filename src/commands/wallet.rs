use serde::Deserialize;
use serde_json::json;

use crate::error::AppResult;
use crate::rpc::RpcClient;

#[derive(Deserialize, Debug)]
pub struct WalletInfo {
    #[serde(rename = "walletname")]
    pub wallet_name: String,
    pub balance: f64,
    pub unconfirmed_balance: f64,
    #[serde(rename = "txcount")]
    pub tx_count: u64,
}

pub fn wallet_info(client: &RpcClient) -> AppResult<()> {
    let info: WalletInfo = client.call_typed("getwalletinfo", json!([]))?;

    println!("Wallet name:        {}", info.wallet_name);
    println!("Balance:            {} BTC", info.balance);
    println!("Unconfirmed balance:{} BTC", info.unconfirmed_balance);
    println!("Transaction count:  {}", info.tx_count);

    Ok(())
}

pub fn balance(client: &RpcClient) -> AppResult<()> {
    let balance: f64 = client.call_typed("getbalance", json!([]))?;
    println!("{balance} BTC");
    Ok(())
}