use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::{Serialize, Deserialize};
use crate::wallet::Wallet;
// Transactions and mempool

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub signature: Option<Vec<u8>>,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: f64) -> Self {
        Transaction { sender, receiver, amount, signature: None }
    }

    fn payload(&self) -> String {
        format!("{}{}{}", self.sender, self.receiver, self.amount)
    }

    pub fn sign(&mut self, wallet: &Wallet) {
        let sig = wallet.sign(self.payload().as_bytes());
        self.signature = Some(sig.to_bytes().to_vec());
    }

    pub fn is_valid(&self) -> bool {
        let Some(sig_bytes) = &self.signature else { return false; };
        let sender_bytes = hex::decode(&self.sender).unwrap_or_default();
        if sender_bytes.len() != 32 { return false; }

        let key_array: [u8; 32] = sender_bytes.try_into().unwrap();
        let verifying_key = match VerifyingKey::from_bytes(&key_array) {
            Ok(k) => k,
            Err(_) => return false,
        };
        let sig_array: [u8; 64] = match sig_bytes.as_slice().try_into() {
            Ok(a) => a,
            Err(_) => return false,
        };
        let signature = Signature::from_bytes(&sig_array);
        verifying_key.verify(self.payload().as_bytes(), &signature).is_ok()
    }
}

#[cfg(test)]
// In src/transaction.rs tests:

mod tests {
    use super::*;
    #[test]
    fn test_signed_transaction_is_valid() {
        let wallet = crate::wallet::Wallet::new();
        let receiver = crate::wallet::Wallet::new();
        let mut tx = Transaction::new(wallet.address(), receiver.address(), 42.0);
        tx.sign(&wallet);
        assert!(tx.is_valid());
    }

    #[test]
    fn test_unsigned_transaction_is_invalid() {
        let tx = Transaction::new("sender".to_string(), "receiver".to_string(), 10.0);
        assert!(!tx.is_valid());
    }
}
