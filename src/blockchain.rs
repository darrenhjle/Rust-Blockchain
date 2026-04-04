use crate::block::Block;
use crate::transaction::Transaction;
use crate::merkle::compute_merkle_root;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

//Blockchain struct that holds a chain of blocks
//Creates genesis block and validates the whole chain

#[derive(Debug, Serialize, Deserialize)]

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis = Block::new(0, "genesis".to_string(), "0".to_string());

        Blockchain {chain :vec![genesis], difficulty: 3} // creates vector and puts genesis block inside of it
    }

    pub fn last_block(&self) -> &Block {
        self.chain.last().expect("Chain is empty")
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let tx_strings: Vec<String> = transactions.iter().map(|tx|serde_json::to_string(tx).unwrap_or_default()).collect();
        let merkle_root = compute_merkle_root(&tx_strings);
        let prev_hash = self.last_block().hash.clone();
        let index = self.chain.len() as u64;
        let mut block = Block::new(index, merkle_root, prev_hash);
        block.mine(self.difficulty);
        self.chain.push(block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let prev = &self.chain[i-1];

            if !current.is_valid() {
                return false;
            }
            if current.prev_hash != prev.hash {
                return false
            }
        }
        true
    }
}

//Save and load the chain from disk so state survives between runs

impl Blockchain {
    pub fn save(&self, path: &str) {
        let json = serde_json::to_string_pretty(self).expect("Serialization failed");
        fs::write(path, json).expect("Failed to write");
    }

    pub fn load(path: &str) ->Self {
        if Path::new(path).exists() {
            let json = fs::read_to_string(path).expect("Failed to read path");
            if json.trim().is_empty() {
                return Self::new();
            }
            serde_json::from_str(&json).expect("Failed to deserialize chain")
        } else {
            Self::new()
        }
    }
}

//#[cfg(test)]

/* mod tests {
    use super::*;

    #[test]
    fn test_genesis_start() {
        let bc = Blockchain::new();
        assert_eq!(bc.chain.len(), 1);
        assert_eq!(bc.chain[0].data, "genesis");
    }

    #[test]
    fn test_valid_chain() {
        let mut bc = Blockchain::new();
        bc.add_block("Block_1".to_string());
        bc.add_block("Block_2".to_string());
        assert!(bc.is_valid());
    }

    #[test]
    fn test_tampered_invalid() {
        let mut bc = Blockchain::new();
        bc.add_block("Block_1".to_string());
        bc.add_block("Block_2".to_string());
        bc.chain[1].data = "tampered".to_string();
        assert!(!bc.is_valid());
    }

    #[test]
    fn test_pow() {
        let mut bc = Blockchain::new();
        bc.add_block("Block 1".to_string());
        let target = "0".repeat(bc.difficulty);
        assert!(bc.last_block().hash.starts_with(&target));
    }

} */