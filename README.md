# rust_blockchain

A blockchain implementation built from scratch in Rust. Supports wallet creation, Ed25519-signed transactions, a persistent mempool, proof-of-work mining, Merkle tree transaction integrity, and a P2P network layer — all driven from the command line.

---

## Features

- SHA-256 block hashing with chain linkage
- Proof-of-work mining with configurable difficulty
- Ed25519 wallet key pairs for transaction signing and verification
- Merkle tree for transaction integrity per block
- Persistent mempool — pending transactions survive between process runs
- Chain and mempool state saved to disk as JSON
- P2P networking over TCP using tokio
- Nodes sync chain on startup and relay new blocks and transactions to peers
- Simple CLI built with clap

---

## Requirements

- Rust 1.85 or later (edition 2024)

---

## Building

```bash
git clone https://github.com/darrenhjle/rust_blockchain.git
cd rust_blockchain
cargo build --release
```

---

## Usage

All commands are run via `cargo run --` or the compiled binary.

### Create a wallet

Generates a new Ed25519 key pair and saves it to `wallet.json`.

```bash
cargo run -- wallet
```

Output:
```
Address:     a3f9c1...
Wallet saved to wallet.json
```

### Send a transaction (local mempool)

Creates a signed transaction and adds it to the local pending mempool (`mempool.json`).

```bash
cargo run -- send --to <recipient-address> --amount <value>
```

### Broadcast a transaction to a running node

Signs a transaction and sends it directly to a running node. The node validates it, adds it to its mempool, and relays it to its peers.

```bash
cargo run -- broadcast-tx --to <recipient-address> --amount <value> --node 127.0.0.1:8000
```

### Mine a block

Drains all pending transactions from the local mempool, mines a new block, and appends it to the chain.

```bash
cargo run -- mine
```

Output:
```
Mining block with 2 transactions...
Mined block 1 | nonce: 14823 | hash: 000a3f9c1d...
Block mined! Chain length: 2
```

### Print the chain

Prints a summary of every block in the chain.

```bash
cargo run -- print
```

Output:
```
#0 | hash: 000abc12... | merkle_root: genesis
#1 | hash: 000f3a91... | merkle_root: a1b2c3d4...
```

### Start a P2P node

Starts a node that listens for incoming connections on the given address. On startup it contacts each known peer and syncs the longest valid chain. Accepts `--peers` multiple times for multiple peers.

```bash
cargo run -- start --address 127.0.0.1:8000 --peers 127.0.0.1:8001
```

To run a two-node network locally, open two terminals:

```bash
# Terminal 1
cargo run -- start --address 127.0.0.1:8000

# Terminal 2 — syncs from node A on startup
cargo run -- start --address 127.0.0.1:8001 --peers 127.0.0.1:8000
```

---

## P2P Message Types

| Message | Direction | Description |
|---|---|---|
| `Ping` | any | Reports current chain height to a peer |
| `RequestChain` | outbound | Asks a peer for its full chain |
| `ResponseChain` | inbound | Receives a peer's full chain; replaces local if longer and valid |
| `NewBlock` | both | Announces a newly mined block; relayed to all peers if valid |
| `NewTransaction` | both | Announces a new transaction; added to mempool and relayed if valid |

---

## Persistence

State is stored in three files in the working directory:

| File | Contents |
|---|---|
| `wallet.json` | Hex-encoded Ed25519 private key |
| `mempool.json` | Pending transactions waiting to be mined |
| `chain.json` | Full blockchain state |

These files are created automatically on first use.

---

## Project Structure

```
src/
├── main.rs             CLI entry point
├── lib.rs              Module declarations
├── block.rs            Block struct, SHA-256 hashing, proof-of-work
├── blockchain.rs       Chain management and disk persistence
├── transaction.rs      Transaction struct, signing, and validation
├── mempool.rs          Pending transaction pool with disk persistence
├── merkle.rs           Merkle tree computation for block transaction root
├── wallet.rs           Ed25519 key pair, address derivation, sign/verify
└── network/
    ├── mod.rs          Network module declarations
    ├── message.rs      P2P message types (serde serialized)
    └── node.rs         TCP node — listen, connect, broadcast, chain sync
```

---

## How It Works

1. **Wallet** — An Ed25519 key pair is generated. The public key (hex-encoded) serves as the wallet address.
2. **Transaction** — A transaction records sender, receiver, and amount. It is signed with the sender's private key and validated against their public key before entering the mempool.
3. **Mempool** — Validated transactions are queued in `mempool.json` and persist until mined.
4. **Mining** — All pending transactions are serialized and hashed into a Merkle root, which is stored in the block. The miner increments a nonce until the block's SHA-256 hash has the required number of leading zeros.
5. **Chain** — Each block references the previous block's hash, forming a tamper-evident chain. Any modification to a block invalidates all subsequent hashes.
6. **P2P** — Each node listens on a TCP port. On startup it requests the chain from known peers and adopts the longest valid one. Incoming blocks and transactions are validated before being accepted and relayed to other peers.

---

## Planned Extensions

- **Mine-and-broadcast** — `mine` currently writes to local disk but does not notify the running node to broadcast the new block. The fix is either a `broadcast-block` command (same pattern as `broadcast-tx`) that connects to the running node's port and sends a `NewBlock` message, or merging mining into the node itself so it mines and broadcasts in one step.
- UTXO-based balance model
- Adjustable difficulty retargeting
- REST API

---

## License

MIT
