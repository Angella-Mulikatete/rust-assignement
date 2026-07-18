mod cli;
mod commands;
mod config;
mod error;
mod rpc;

use clap::Parser;
use serde_json::Value;

use cli::{Cli, Command};
use config::Config;
use error::AppError;
use rpc::RpcClient;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    let cli = Cli::parse();
    let config = Config::from_env()?;
    let client = RpcClient::new(config);

    match cli.command {
        Command::BlockchainInfo => commands::blockchain::run(&client),
        Command::WalletInfo => commands::wallet::wallet_info(&client),
        Command::Balance => commands::wallet::balance(&client),
        Command::NewAddress => commands::address::run(&client),
        Command::Rpc { method, params } => run_generic_rpc(&client, &method, params),
    }
}


fn run_generic_rpc(client: &RpcClient, method: &str, params: Vec<String>) -> Result<(), AppError> {
    let json_params: Vec<Value> = params
        .into_iter()
        .map(|p| serde_json::from_str(&p).unwrap_or(Value::String(p)))
        .collect();

    let result = client.call(method, Value::Array(json_params))?;
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
    Ok(())
}