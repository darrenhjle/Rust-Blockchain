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
    pub merkle_root: String,
    pub nonce: u64,
    pub prev_hash: String,
    pub hash: String,
}

//Create a block and compute its hash
impl Block {

    pub fn new(index: u64, merkle_root: String, prev_hash: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let nonce = 0;
        let hash = Self::compute_hash(index, timestamp, &merkle_root, &prev_hash, nonce);

        Block {index, timestamp, merkle_root, nonce, prev_hash, hash }
    }

    pub fn compute_hash(index: u64, 
        timestamp: i64,
        merkle_root: &str,
        prev_hash: &str,
        nonce: u64,
    ) -> String {
        let input = format!("{}{}{}{}{}", index, timestamp, merkle_root, prev_hash, nonce);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());

        hex::encode(hasher.finalize())
    }

    pub fn is_valid(&self) -> bool {
        let expected = Self::compute_hash(
            self.index, self.timestamp, &self.merkle_root, &self.prev_hash, self.nonce
        );

        self.hash == expected

    }

    //Proof of work portion
    //Target is a string of zeros, number of zeros is determined by the difficulty
    //Increases the nonce by 1 until the computed hash starts with the same nubmer of zeros as the target
    pub fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = Self::compute_hash(self.index, self.timestamp, &self.merkle_root, &self.prev_hash, self.nonce);
        }
        println!("Mined block {} | nonce: {} | hash: {}", self.index, self.nonce, &self.hash[..10]);

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
        block.merkle_root = "tampered".to_string(); // change data without recomputing hash
        assert!(!block.is_valid());
    }
}