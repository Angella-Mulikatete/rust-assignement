use serde::Deserialize;
use serde_json::json;

use crate::error::AppResult;
use crate::rpc::RpcClient;

#[derive(Deserialize, Debug)]
struct WalletInfoRaw {
    #[serde(rename = "walletname")]
    wallet_name: String,
    #[serde(rename = "txcount")]
    tx_count: u64,
}

#[derive(Deserialize, Debug)]
struct BalancesMine {
    trusted: f64,
    untrusted_pending: f64,
}

#[derive(Deserialize, Debug)]
struct Balances {
    mine: BalancesMine,
}

pub fn wallet_info(client: &RpcClient) -> AppResult<()> {
    let info: WalletInfoRaw = client.call_typed("getwalletinfo", json!([]))?;
    let balances: Balances = client.call_typed("getbalances", json!([]))?;

    println!("Wallet name:         {}", info.wallet_name);
    println!("Balance:             {} BTC", balances.mine.trusted);
    println!("Unconfirmed balance: {} BTC", balances.mine.untrusted_pending);
    println!("Transaction count:   {}", info.tx_count);

    Ok(())
}

pub fn balance(client: &RpcClient) -> AppResult<()> {
    let balance: f64 = client.call_typed("getbalance", json!([]))?;
    println!("{balance} BTC");
    Ok(())
}