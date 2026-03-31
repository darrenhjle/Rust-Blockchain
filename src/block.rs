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
        let hash = Self.compute_hash(index, timestamp, &data, nonce, &prev_hash);

        Block {index, timestamp, data, nonce, prev_hash, hash }
    }

    pub fn compute_hash(index: u64, 
        timestamp: i64,
        data: &str,
        nonce: u64,
        prev_hash: &str,
    ) -> String {
        //computes hash
        data 
    }
}
