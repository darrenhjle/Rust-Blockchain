# Progress Tracker

## Phase 1 — Block & Hashing
- [x] Block struct defined
- [x] SHA-256 hashing works
- [x] Genesis block creates correctly
- [x] All Phase 1 tests pass

## Phase 2 — Blockchain & PoW
- [x] Blockchain struct with Vec<Block>
- [x] add_block() implemented
- [x] Proof of Work mining loop works
- [x] Chain validation works
- [x] All Phase 2 tests pass

## Phase 3 — Wallets & Transactions
- [x] Key pair generation works
- [x] Transaction struct defined
- [x] Signing and verification works
- [x] Mempool stores pending transactions
- [x] All Phase 3 tests pass

## Phase 4 — CLI & Persistence
- [x] clap CLI with mine/balance/send commands
- [x] Chain serializes to chain.json
- [x] Chain loads from chain.json on startup
- [x] End-to-end flow works

## Future extentsions
- P2P networking with tokio + TCP
- Merkle tree for transactions
- UTXO balance model
- Adjustable difficulty retargeting like BTC 2-week adjustment
- REST API