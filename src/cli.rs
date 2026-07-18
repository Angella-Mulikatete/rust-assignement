use clap::{Parser, Subcommand};

/// A command-line tool for interacting with a Bitcoin Core node via JSON-RPC.
#[derive(Parser, Debug)]
#[command(name = "rust-bitcoin-cli", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Show chain, block height, headers, difficulty, and verification progress
    BlockchainInfo,

    /// Show wallet name, balance, unconfirmed balance, and transaction count
    WalletInfo,

    Balance,
    NewAddress,
    Rpc {
        method: String,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        params: Vec<String>,
    },
}