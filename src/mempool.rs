use crate::transaction::Transaction;
use std::fs;
use std::path::Path;

#[derive(Debug, Default)]

pub struct Mempool {
    pub pending: Vec<Transaction>,
}

impl Mempool {
    pub fn new() -> Self {
        Mempool {pending: vec![]}
    }

    pub fn load(path: &str) -> Self {
        if Path::new(path).exists() {
            let json = fs::read_to_string(path).expect("Failed to read mempool");
            let pending: Vec<Transaction> = serde_json::from_str(&json).unwrap_or_default();
            Mempool { pending }
        } else {
            Self::new()
        }
    }

    pub fn save(&self, path: &str) {
        let json = serde_json::to_string_pretty(&self.pending).expect("Serialization failed");
        fs::write(path, json).expect("Failed to write mempool");
    }

    pub fn add(&mut self, tx: Transaction) -> bool {
        if tx.is_valid() {
            self.pending.push(tx);
            true
        }
        else {
            false
        }
    }

    pub fn drain(&mut self) -> Vec<Transaction> {
        std::mem::take(&mut self.pending)
    }
}