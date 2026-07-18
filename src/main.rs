mod cli;
mod commands;
mod config;
mod error;
mod rpc;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli.command);
}