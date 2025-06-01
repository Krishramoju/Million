// libs/neuro_utils/src/memory.rs

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

// Type alias for a memory token embedding vector
pub type Embedding = Vec<f32>;

// A single memory entry stored in NeuroOS memory system
#[derive(Clone, Debug)]
pub struct MemoryEntry {
    pub id: u64,
    pub timestamp: u64,
    pub context: String,
    pub embedding: Embedding,
    pub metadata: HashMap<String, String>,
}

// Memory Manager to handle vector-based memory storage with a fixed capacity
pub struct MemoryManager {
    capacity: usize,
    storage: VecDeque<MemoryEntry>,
    lookup: HashMap<u64, usize>, // Maps id -> index in storage
    next_id: u64,
}

impl MemoryManager {
    // Initialize a new MemoryManager with fixed capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            storage: VecDeque::with_capacity(capacity),
            lookup: HashMap::new(),
            next_id: 1,
        }
    }

    // Add a new memory entry, evict oldest if capacity exceeded
    pub fn add_memory(&mut self, context: String, embedding: Embedding, metadata: HashMap<String, String>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        if self.storage.len() == self.capacity {
            if let Some(oldest) = self.storage.pop_front() {
                self.lookup.remove(&oldest.id);
            }
        }

        let entry = MemoryEntry {
            id,
            timestamp: Self::current_timestamp(),
            context,
            embedding,
            metadata,
        };

        self.storage.push_back(entry);
        self.lookup.insert(id, self.storage.len() - 1);

        id
    }

    // Retrieve a memory entry by its id
    pub fn get_memory(&self, id: u64) -> Option<&MemoryEntry> {
        self.lookup.get(&id).and_then(|&idx| self.storage.get(idx))
    }

    // Search memories by similarity score (cosine similarity)
    pub fn search_similar(&self, query_embedding: &Embedding, threshold: f32) -> Vec<&MemoryEntry> {
        self.storage.iter()
            .filter(|entry| cosine_similarity(&entry.embedding, query_embedding) >= threshold)
            .collect()
    }

    // Simple helper to get current timestamp (epoch seconds)
    fn current_timestamp() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }
}

// Compute cosine similarity between two vectors
pub fn cosine_similarity(a: &Embedding, b: &Embedding) -> f32 {
    let dot = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum::<f32>();
    let mag_a = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let mag_b = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if mag_a == 0.0 || mag_b == 0.0 {
        0.0
    } else {
        dot / (mag_a * mag_b)
    }
}

// Thread-safe shared memory manager type alias
pub type SharedMemoryManager = Arc<Mutex<MemoryManager>>;
