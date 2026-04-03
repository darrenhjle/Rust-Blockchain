use sha2::{Sha256, Digest};
use hex;

pub fn compute_merkle_root(transactions: &[String]) -> String {
    if transactions.is_empty() {
        return String::from("0".repeat(64));
    }

    //leaf nodes
    let mut hashes: Vec<String> = transactions.iter().map(|tx| hash(tx)).collect();

    while hashes.len()>1 {
         //handling odd numbers
        //Every node needs a partner. If there is an odd number of nodes, clone the last node and add it
        if hashes.len() % 2 != 0{
            let last = hashes.last().unwrap().clone();
            hashes.push(last);
        } 

        //chunk(2) groups the hashes into pairs
        hashes = hashes.chunks(2).map(|pair| hash(&format!("{}{}", pair[0], pair[1]))).collect();

    }
    hashes[0].clone()
}

fn hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    hex::encode(hasher.finalize())
}