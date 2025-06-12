use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u32, data: String, previous_hash: String) -> Self {
        let timestamp = current_time();
        let hash = calculate_hash(index, timestamp, &data, &previous_hash);
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }
}

pub fn calculate_hash(index: u32, timestamp: u128, data: &str, previous_hash: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(index.to_string());
    hasher.update(timestamp.to_string());
    hasher.update(data);
    hasher.update(previous_hash);
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn current_time() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        Blockchain {
            chain: vec![genesis],
        }
    }

    pub fn add_block(&mut self, data: String) {
        let prev = self.chain.last().unwrap();
        let block = Block::new(
            prev.index + 1,
            data,
            prev.hash.clone(),
        );
        self.chain.push(block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            let recalculated_hash = calculate_hash(
                current.index,
                current.timestamp,
                &current.data,
                &current.previous_hash,
            );

            if current.hash != recalculated_hash {
                println!("❌ Tampered block at index {}", i);
                return false;
            }

            if current.previous_hash != previous.hash {
                println!("❌ Broken chain at index {}", i);
                return false;
            }
        }
        true
    }

    pub fn print_chain(&self) {
        for block in &self.chain {
            println!("{:#?}", block);
        }
    }
}
