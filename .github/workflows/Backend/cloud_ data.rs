use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Nonce}; // AES-GCM encryption
use sha2::{Sha256, Digest};

const NONCE_SIZE: usize = 12;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CloudApp {
    ChatBot,
    BrainTeaser,
    TeamRecruitment,
}

#[derive(Debug)]
pub struct CloudDataRecord {
    pub encrypted_data: Vec<u8>,
    pub hash: String,
    pub owner_id: String,
    pub app: CloudApp,
    pub timestamp: u64,
}

pub struct CloudDataStorage {
    pub records: Arc<Mutex<HashMap<String, CloudDataRecord>>>, // id -> record
    pub encryption_key: [u8; 32], // 256-bit AES key
}

impl CloudDataStorage {
    pub fn new() -> Self {
        let key = Aes256Gcm::generate_key(&mut OsRng);
        CloudDataStorage {
            records: Arc::new(Mutex::new(HashMap::new())),
            encryption_key: key.into(),
        }
    }

    pub fn store_data(&self, id: &str, plaintext: &str, user: &str, app: CloudApp) -> Result<(), String> {
        let cipher = Aes256Gcm::new_from_slice(&self.encryption_key).unwrap();
        let nonce = aes_gcm::Nonce::from_slice(&Self::generate_nonce()); // 96-bit nonce

        let encrypted_data = cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| format!("Encryption failed: {:?}", e))?;

        let hash = Self::hash_data(plaintext);
        let timestamp = Self::current_timestamp();

        let record = CloudDataRecord {
            encrypted_data,
            hash,
            owner_id: user.to_string(),
            app,
            timestamp,
        };

        self.records.lock().unwrap().insert(id.to_string(), record);
        Ok(())
    }

    pub fn retrieve_data(&self, id: &str, user: &str, app: &CloudApp) -> Result<String, String> {
        let records = self.records.lock().unwrap();
        let record = records.get(id).ok_or("Data not found")?;

        if &record.owner_id != user || &record.app != app {
            return Err("Access denied".to_string());
        }

        let cipher = Aes256Gcm::new_from_slice(&self.encryption_key).unwrap();
        let nonce = aes_gcm::Nonce::from_slice(&Self::generate_nonce()); // For now, fixed nonce for demo; you should store the nonce!

        let decrypted_bytes = cipher
            .decrypt(nonce, record.encrypted_data.as_ref())
            .map_err(|_| "Decryption failed")?;

        let decrypted_str = String::from_utf8(decrypted_bytes).map_err(|_| "Invalid UTF-8")?;

        // Integrity check
        let integrity_hash = Self::hash_data(&decrypted_str);
        if integrity_hash != record.hash {
            return Err("Data integrity check failed".to_string());
        }

        Ok(decrypted_str)
    }

    fn hash_data(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn generate_nonce() -> [u8; NONCE_SIZE] {
        // NOTE: In production, store a random nonce with each record!
        [0u8; NONCE_SIZE] // Placeholder
    }

    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
