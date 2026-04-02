use crate::block::Block;
use serde::{Serialize, Deserialize};

//Blockchain struct that holds a chain of blocks
//Creates genesis block and validates the whole chain

#[derive(Debug, Serialize, Deserialize)]

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis = Block::new(0, "genesis".to_string(), "0".to_string());

        Blockchain {chain :vec![genesis], difficulty: 3} // creates vector and puts genesis block inside of it
    }

    pub fn last_block(&self) -> &Block {
        self.chain.last().expect("Chain is empty")
    }

    pub fn add_block(&mut self, data: String) {
        let prev_hash = self.last_block().hash.clone();
        let index = self.chain.len() as u64;
        let mut block = Block::new(index, data, prev_hash);
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


#[cfg(test)]

mod tests {
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

}