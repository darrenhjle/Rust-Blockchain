use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;


//proves who you are
//authorize transactions

pub struct Wallet {
    pub signing_key: SigningKey, //private key
    pub verifying_key: VerifyingKey, //public key
}

impl Wallet {
    pub fn new() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        Wallet {signing_key, verifying_key}
    }

    //Returns hex encoded public key
    pub fn address(&self) -> String {
        hex::encode(self.verifying_key.as_bytes())
    }

    //Signature = message + signing key
    pub fn sign(&self, message: &[u8]) -> Signature {
        self.signing_key.sign(message)
    }

     // Verifies a signature against a message and public key bytes.
     // Checking if the signature produced from the message using the private key matches the message and the public key
    pub fn verify(pubkey_bytes: &[u8], message: &[u8], signature: &Signature) -> bool {
        let key_array: [u8; 32] = pubkey_bytes.try_into().expect("Invalid key length");
        let verifying_key = VerifyingKey::from_bytes(&key_array).expect("Invalid key");
        verifying_key.verify(message, signature).is_ok()
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_sign_verify() {
        let wallet = Wallet::new();
        let message = b"send 5 coins";
        let sig = wallet.sign(message);
        assert!(Wallet::verify(wallet.verifying_key.as_bytes(), message, &sig))
    }

    #[test]
    fn test_wrong_message_fails_verification() {
        let wallet = Wallet::new();
        let sig = wallet.sign(b"real message");
        assert!(!Wallet::verify(wallet.verifying_key.as_bytes(), b"fake message", &sig));
    }
}