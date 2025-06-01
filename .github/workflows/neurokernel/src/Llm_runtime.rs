use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::tokenizer::{Tokenizer, Token};
use crate::memory_router::{MemoryRouter, MemorySegment};
use crate::emotion_infer::EmotionState;
use crate::intent_negotiator::Intent;

pub struct LLMSession {
    pub id: String,
    pub context_tokens: Vec<Token>,
    pub memory: Arc<Mutex<MemoryRouter>>,
    pub emotion_state: EmotionState,
    pub last_accessed: Instant,
    pub intents: Vec<Intent>,
}

pub struct LLMScheduler {
    pub sessions: HashMap<String, LLMSession>,
    pub tokenizer: Tokenizer,
}

impl LLMScheduler {
    pub fn new() -> Self {
        LLMScheduler {
            sessions: HashMap::new(),
            tokenizer: Tokenizer::new(),
        }
    }

    pub fn create_session(&mut self, session_id: &str) {
        let memory = Arc::new(Mutex::new(MemoryRouter::new()));
        let emotion_state = EmotionState::neutral();
        let session = LLMSession {
            id: session_id.to_string(),
            context_tokens: Vec::new(),
            memory,
            emotion_state,
            last_accessed: Instant::now(),
            intents: Vec::new(),
        };
        self.sessions.insert(session_id.to_string(), session);
    }

    pub fn process_input(&mut self, session_id: &str, input: &str) -> Result<String, String> {
        let session = self.sessions.get_mut(session_id)
            .ok_or_else(|| format!("Session {} not found", session_id))?;

        // Tokenize input
        let tokens = self.tokenizer.tokenize(input);

        // Append tokens to context
        session.context_tokens.extend(tokens.clone());

        // Update last accessed
        session.last_accessed = Instant::now();

        // Update Memory
        {
            let mut mem = session.memory.lock().unwrap();
            mem.integrate_tokens(&tokens);
        }

        // Infer emotion state from input
        session.emotion_state = EmotionState::from_text(input);

        // Negotiate intent
        let intent = Intent::from_text(input);
        session.intents.push(intent.clone());

        // Generate output tokens (mock example)
        let response_tokens = self.generate_response(&session.context_tokens, &session.memory);

        // Convert tokens back to string
        let response = self.tokenizer.detokenize(&response_tokens);

        Ok(response)
    }

    fn generate_response(&self, context: &Vec<Token>, memory: &Arc<Mutex<MemoryRouter>>) -> Vec<Token> {
        // Placeholder for actual transformer inference with memory routing
        // Here, simply echo last 10 tokens reversed + a token for [END]
        let slice_len = std::cmp::min(10, context.len());
        let mut response = context[context.len() - slice_len..].to_vec();
        response.reverse();
        response.push(self.tokenizer.end_token());
        response
    }

    pub fn cleanup_sessions(&mut self, timeout_secs: u64) {
        let now = Instant::now();
        self.sessions.retain(|_, session| {
            now.duration_since(session.last_accessed) < Duration::from_secs(timeout_secs)
        });
    }
}
