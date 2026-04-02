# rust_blockchain

A blockchain implementation built from scratch in Rust. Supports wallet creation, Ed25519-signed transactions, a persistent mempool, proof-of-work mining, and chain inspection — all driven from the command line.

---

## Features

- SHA-256 block hashing with chain linkage
- Proof-of-work mining with configurable difficulty
- Ed25519 wallet key pairs for transaction signing and verification
- Persistent mempool — pending transactions survive between process runs
- Chain and mempool state saved to disk as JSON
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

### Send a transaction

Creates a signed transaction and adds it to the pending mempool (`mempool.json`).

```bash
cargo run -- send --to <recipient-address> --amount <value>
```

Example:
```bash
cargo run -- send --to a3f9c1d2e4b5... --amount 10
```

### Mine a block

Drains all pending transactions from the mempool, mines a new block, and appends it to the chain.

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
#0 | hash: 000abc12... | data: genesis
#1 | hash: 000f3a91... | data: [{"sender":"a3f9...
```

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
├── main.rs         CLI entry point
├── lib.rs          Module declarations
├── block.rs        Block struct, SHA-256 hashing, proof-of-work
├── blockchain.rs   Chain management and disk persistence
├── transaction.rs  Transaction struct, signing, and validation
├── mempool.rs      Pending transaction pool with disk persistence
└── wallet.rs       Ed25519 key pair, address derivation, sign/verify
```

See [docs/architecture.md](docs/architecture.md) for a full breakdown of each component.

---

## How It Works

1. **Wallet** — An Ed25519 key pair is generated. The public key (hex-encoded) serves as the wallet address.
2. **Transaction** — A transaction records sender, receiver, and amount. It is signed with the sender's private key and validated against their public key before entering the mempool.
3. **Mempool** — Validated transactions are queued in `mempool.json` and persist until mined.
4. **Mining** — All pending transactions are serialized into a block's data field. The miner increments a nonce until the block's SHA-256 hash has the required number of leading zeros.
5. **Chain** — Each block references the previous block's hash, forming a tamper-evident chain. Any modification to a block invalidates all subsequent hashes.

---

## Planned Extensions

- P2P networking with tokio and TCP
- Merkle tree for transaction integrity
- UTXO-based balance model
- Adjustable difficulty retargeting
- REST API

---

## License

MIT
