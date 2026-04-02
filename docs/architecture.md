# Architecture

## Overview

`rust_blockchain` is a CLI-driven, file-persisted blockchain written in Rust. It supports wallet creation, transaction signing, a pending transaction mempool, proof-of-work mining, and chain inspection.

---

## Module Structure

```
src/
├── lib.rs          — re-exports all public modules
├── main.rs         — CLI entry point (clap)
├── block.rs        — Block struct, hashing, PoW
├── blockchain.rs   — Chain management, disk persistence
├── transaction.rs  — Transaction struct, signing, validation
├── mempool.rs      — Pending transaction pool, disk persistence
└── wallet.rs       — Ed25519 key pair, address, sign/verify
```

---

## Components

### `Block` (`block.rs`)

Fields: `index`, `timestamp`, `data` (JSON-serialized transactions), `nonce`, `prev_hash`, `hash`.

- `Block::new()` — constructs a block and computes its initial SHA-256 hash.
- `Block::compute_hash()` — deterministic hash of `index + timestamp + data + prev_hash + nonce`.
- `Block::mine(difficulty)` — increments `nonce` until the hash has `difficulty` leading zeros (proof of work).
- `Block::is_valid()` — recomputes and compares the hash to detect tampering.

### `Blockchain` (`blockchain.rs`)

Fields: `chain: Vec<Block>`, `difficulty: usize` (currently `3`).

- `Blockchain::new()` — initializes with a hardcoded genesis block (`data = "genesis"`, `prev_hash = "0"`).
- `Blockchain::add_block(txs)` — serializes the transaction list to JSON, creates a new block, mines it, and appends it.
- `Blockchain::is_valid()` — walks the chain verifying each block's hash and `prev_hash` linkage.
- `Blockchain::save(path)` / `Blockchain::load(path)` — JSON persistence to `chain.json`.

### `Transaction` (`transaction.rs`)

Fields: `sender` (hex public key), `receiver` (hex public key), `amount: f64`, `signature: Option<Vec<u8>>`.

- `Transaction::new()` — creates an unsigned transaction.
- `Transaction::sign(wallet)` — signs the payload `sender + receiver + amount` with the wallet's Ed25519 private key.
- `Transaction::is_valid()` — verifies the signature against the sender's public key. Rejects unsigned or malformed transactions.
- Payload used for signing: `format!("{}{}{}", sender, receiver, amount)`.

### `Mempool` (`mempool.rs`)

Fields: `pending: Vec<Transaction>`.

- `Mempool::load(path)` / `Mempool::save(path)` — JSON persistence to `mempool.json`, so pending transactions survive between process invocations.
- `Mempool::add(tx)` — validates the transaction before adding; rejects invalid ones.
- `Mempool::drain()` — moves all pending transactions out (used by `mine`).

### `Wallet` (`wallet.rs`)

Fields: `signing_key: SigningKey` (private), `verifying_key: VerifyingKey` (public).

- `Wallet::new()` — generates a fresh Ed25519 key pair using OS randomness.
- `Wallet::address()` — hex-encoded 32-byte public key, used as the identity/sender address.
- `Wallet::sign(message)` — returns an Ed25519 `Signature`.
- `Wallet::verify(pubkey_bytes, message, signature)` — static verifier used by `Transaction::is_valid()`.
- `Wallet::save(path)` / `Wallet::load(path)` — stores/loads the raw 32-byte private key as hex in `wallet.json`.

---

## CLI (`main.rs`)

Built with `clap` derive macros.

| Command | Behavior |
|---|---|
| `wallet` | Generates a new Ed25519 wallet, saves to `wallet.json`, prints address |
| `send --to <addr> --amount <n>` | Loads `wallet.json`, creates and signs a transaction, adds it to mempool, saves `mempool.json` |
| `mine` | Drains mempool, mines a new block containing those transactions, saves `chain.json` and clears `mempool.json` |
| `print` | Iterates `chain.json` and prints index, hash prefix, and data preview for each block |

---

## Data Flow

```
wallet.json  ──load──>  Wallet
                           │
                        sign Tx
                           │
mempool.json <──save──  Mempool  ──drain──> Blockchain::add_block()
                                                    │
                                                 mine (PoW)
                                                    │
chain.json  <──────────────────────────────── Blockchain::save()
```

---

## Persistence Files

| File | Contents |
|---|---|
| `wallet.json` | Hex-encoded 32-byte Ed25519 private key |
| `mempool.json` | JSON array of pending `Transaction` objects |
| `chain.json` | Full serialized `Blockchain` (chain + difficulty) |

---

## Dependencies

| Crate | Purpose |
|---|---|
| `sha2` | SHA-256 block hashing |
| `hex` | Encode/decode keys and hashes |
| `serde` / `serde_json` | Struct serialization for persistence |
| `chrono` | UTC timestamps on blocks |
| `ed25519-dalek` | Ed25519 key generation, signing, verification |
| `rand` | OS-backed RNG for key generation |
| `clap` | CLI argument parsing |
| `thiserror` | Error type derivation (available, not yet used) |
