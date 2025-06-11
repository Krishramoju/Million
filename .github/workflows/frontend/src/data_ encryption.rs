use aes_gcm::aead::{Aead, KeyInit, OsRng, generic_array::GenericArray};
use aes_gcm::{Aes256Gcm, Nonce}; // 96-bit nonce
use rand::RngCore;
use base64::{encode, decode};

pub struct DataEncryptor {
    key: Aes256Gcm,
}

impl DataEncryptor {
    // Creates a new encryptor from a 32-byte (256-bit) base64 key
    pub fn new(base64_key: &str) -> Self {
        let key_bytes = decode(base64_key).expect("Invalid base64 key");
        let key = Aes256Gcm::new(GenericArray::from_slice(&key_bytes));
        DataEncryptor { key }
    }

    // Encrypt a string
    pub fn encrypt(&self, plaintext: &str) -> (String, String) {
        let mut nonce_bytes = [0u8; 12]; // 96-bit nonce
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = self.key.encrypt(nonce, plaintext.as_bytes())
            .expect("Encryption failed");

        // Return both ciphertext and nonce (base64-encoded)
        (encode(&ciphertext), encode(&nonce_bytes))
    }

    // Decrypt a string using the nonce
    pub fn decrypt(&self, ciphertext_b64: &str, nonce_b64: &str) -> String {
        let ciphertext = decode(ciphertext_b64).expect("Invalid ciphertext base64");
        let nonce_bytes = decode(nonce_b64).expect("Invalid nonce base64");
        let nonce = Nonce::from_slice(&nonce_bytes);

        let plaintext = self.key.decrypt(nonce, ciphertext.as_ref())
            .expect("Decryption failed");

        String::from_utf8(plaintext).expect("Invalid UTF-8")
    }
}
