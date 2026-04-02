use crate::transaction::Transaction;

#[derive(Debug, Default)]

pub struct Mempool {
    pub pending: Vec<Transaction>,
}

impl Mempool {
    pub fn new() -> Self {
        Mempool {pending: vec![]}
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