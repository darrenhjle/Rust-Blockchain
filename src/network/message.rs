use serde::{Serialize, Deserialize};
use crate::block::Block;
use crate::transaction::Transaction;

#[derive(Debug, Serialize, Deserialize)]

pub enum Message {
    Ping { block_length: u64}, //returns current length of chain
    RequestChain, // request for the full chain
    ResponseChain {chain: Vec<Block>}, // sends full chain
    NewBlock {block: Block}, // add new block to chain
    NewTransaction {transaction: Transaction}, // add new transaction to chain
}