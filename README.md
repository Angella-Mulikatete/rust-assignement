# Rust Bitcoin CLI

A command-line application in Rust that communicates with a local Bitcoin Core
node (running on Regtest via Polar) using the JSON-RPC interface.

## Project Overview

This tool connects to a Bitcoin Core node and exposes wallet and blockchain
information through simple CLI commands, plus a generic passthrough for
arbitrary RPC calls. Configuration (RPC URL, user, password) is supplied via
environment variables or a `.env` file — no credentials are hardcoded.

## Project Structure
```text
src/
├── main.rs           # entry point, config/client wiring, command dispatch
├── cli.rs            # clap CLI definition
├── rpc.rs            # JSON-RPC client (HTTP + auth + error mapping)
├── config.rs         # loads RPC credentials from env / .env
├── error.rs          # unified AppError type
└── commands/
    ├── blockchain.rs # blockchain-info
    ├── wallet.rs     # wallet-info, balance
    └── address.rs    # new-address
```

## Installation

1. Install [Rust](https://www.rust-lang.org/tools/install) (via `rustup`).
2. Clone this repository:
```bash
git clone <your-repo-url>
cd rust-bitcoin-cli
```
3. Build the project:
```bash
cargo build
```

## Setting Up Polar

1. Download and install [Polar](https://lightningpolar.com/) (requires
   [Docker Desktop](https://www.docker.com/products/docker-desktop) on Windows/Mac).
2. Open Polar and click **Create Network**.
3. Ensure the network includes at least one **Bitcoin Core** node.
4. Click **Start** and wait for the node's status light to turn green.

## Running the Bitcoin Core Node

1. In Polar, click on the **bitcoind** node in your network diagram.
2. Open the **Connect** tab to find:
   - RPC host and port (e.g. `http://127.0.0.1:18443`)
   - RPC username and password
3. Bitcoin Core starts with no wallet loaded by default. Create one:
```bash
cargo run -- rpc createwallet mywallet
```

## Configuring the Application

Copy `.env.example` to `.env` and fill in your Polar node's credentials:
```env
RPC_URL=http://127.0.0.1:18443
RPC_USER=polaruser
RPC_PASSWORD=polarpass
WALLET_NAME=mywallet
```

All configuration is read from environment variables (optionally loaded from
`.env` via `dotenvy`). No source code changes are required to point the app
at a different node or wallet.

## Running the Application

```bash
cargo run -- <command>
```

## Example Commands & Output

### blockchain-info
```bash
$ cargo run -- blockchain-info
Chain:                regtest
Blocks:               1
Headers:              1
Difficulty:           0.00000000046565423739069247
Verification progress:100.0000%
```

### wallet-info
```bash
$ cargo run -- wallet-info
Wallet name:         mywallet
Balance:             0 BTC
Unconfirmed balance: 0 BTC
Transaction count:   0
```

### balance
```bash
$ cargo run -- balance
0 BTC
```

### new-address
```bash
$ cargo run -- new-address
bcrt1qyx273a4ak69x0l8l68s8ecd95c5stnyd3g5gxs
```

### Generic RPC passthrough
```bash
$ cargo run -- rpc getblockcount
1

$ cargo run -- rpc getblockhash 0
"0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206"
```

## Error Handling

The application avoids panics and returns clear, user-friendly messages for:
- **Invalid credentials** — returns `Authentication failed` on HTTP 401
- **Connection failures** — reports the RPC URL that could not be reached
- **Invalid RPC methods / parameters** — surfaces Bitcoin Core's own RPC
  error code and message (e.g. `-32601 Method not found`)
- **Missing wallet** — detected via Bitcoin Core's `-18` error code and
  reported as a distinct, specific error

## Assumptions & Design Decisions

- Used the `blocking` feature of `reqwest` rather than async/Tokio, since the
  CLI is single-request-per-invocation and doesn't benefit from concurrency.
- `getwalletinfo`'s `balance`/`unconfirmed_balance` fields are deprecated and
  hidden by default in recent Bitcoin Core versions, so `wallet-info` instead
  calls `getbalances` for balance data and `getwalletinfo` only for wallet
  name and transaction count.
- Generic `rpc` command arguments are parsed as JSON when possible (so
  numbers/booleans are sent as their proper JSON type), falling back to plain
  strings otherwise — this lets a single code path support all RPC methods
  without per-method argument handling.
- Config is loaded once at startup from environment variables / `.env`; no
  hot-reloading of credentials mid-run.

## Suggested Crates Used

`clap`, `serde`, `serde_json`, `reqwest`, `thiserror`, `dotenvy`
