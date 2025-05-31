//! neurokernel/src/vault_storage.rs
//! Encrypted vault for secure storage of user files and secrets

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};

pub struct Vault {
    key: Key<Aes256Gcm>,
    storage_path: String,
    index: HashMap<String, String>,
}

impl Vault {
    pub fn new(key_bytes: &[u8], path: &str) -> Self {
        let key = Key::from_slice(key_bytes);
        Vault {
            key: *key,
            storage_path: path.to_string(),
            index: HashMap::new(),
        }
    }

    pub fn store(&mut self, label: &str, plaintext: &str) {
        let cipher = Aes256Gcm::new(&self.key);
        let nonce = Nonce::from_slice(b"unique_nonce12"); // Must be unique in production

        let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes())
            .expect("encryption failure!");

        let file_path = format!("{}/{}.vault", self.storage_path, label);
        fs::create_dir_all(&self.storage_path).unwrap();
        let mut file = File::create(&file_path).unwrap();
        file.write_all(&ciphertext).unwrap();
        self.index.insert(label.to_string(), file_path);

        println!("🔐 Data for '{}' securely stored.", label);
    }

    pub fn retrieve(&self, label: &str) -> Option<String> {
        if let Some(path) = self.index.get(label) {
            let mut file = File::open(path).ok()?;
            let mut ciphertext = Vec::new();
            file.read_to_end(&mut ciphertext).ok()?;

            let cipher = Aes256Gcm::new(&self.key);
            let nonce = Nonce::from_slice(b"unique_nonce12");
            let plaintext = cipher.decrypt(nonce, ciphertext.as_ref()).ok()?;

            Some(String::from_utf8(plaintext).ok()?)
        } else {
            None
        }
    }

    pub fn list_labels(&self) {
        println!("📁 Stored Entries:");
        for label in self.index.keys() {
            println!("- {}", label);
        }
    }
}
