use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use chrono::Utc;

// A block will have:
// index: its position in the chain
// timestamp: when it was created
// data: payload data
// nonce: number for POW
// previous hash: links it to the previous block
// hash: a SHA256 digest of all the above fields

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
    pub prev_hash: String,
    pub hash: String,
}

//Create a block and compute its hash
impl Block {

    pub fn new(index: u64, data: String, prev_hash: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let nonce = 0;
        let hash = Self::compute_hash(index, timestamp, &data, &prev_hash, nonce);

        Block {index, timestamp, data, nonce, prev_hash, hash }
    }

    pub fn compute_hash(index: u64, 
        timestamp: i64,
        data: &str,
        prev_hash: &str,
        nonce: u64,
    ) -> String {
        let input = format!("{}{}{}{}{}", index, timestamp, data, prev_hash, nonce);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());

        hex::encode(hasher.finalize())
    }

    pub fn is_valid(&self) -> bool {
        let expected = Self::compute_hash(
            self.index, self.timestamp, &self.data, &self.prev_hash, self.nonce
        );

        self.hash == expected

    }
}

//Unit test

#[cfg(test)]

mod tests {
    use super::*;

    #[test]

    fn test_block_is_valid() {
        let block = Block::new(0, "genesis".to_string(), "0".to_string());
        assert!(block.is_valid());
    }

    #[test]
    fn test_tampered_block_is_invalid() {
        let mut block = Block::new(1, "data".to_string(), "abc".to_string());
        block.data = "tampered".to_string(); // change data without recomputing hash
        assert!(!block.is_valid());
    }
}