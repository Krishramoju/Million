use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::tokenizer::{Tokenizer, Token};
use crate::memory_router::{MemoryRouter, MemorySegment};
use crate::emotion_infer::EmotionState;
use crate::intent_negotiator::Intent;

/// A struct representing user preferences for personalized output
#[derive(Clone)]
pub struct UserProfile {
    pub name: String,
    pub prefers_formal: bool,
    pub favorite_topics: Vec<String>,
}

/// Session holds user profile for personalization
pub struct LLMSession {
    pub id: String,
    pub context_tokens: Vec<Token>,
    pub memory: Arc<Mutex<MemoryRouter>>,
    pub emotion_state: EmotionState,
    pub last_accessed: Instant,
    pub intents: Vec<Intent>,
    pub user_profile: UserProfile,
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

    pub fn create_session(&mut self, session_id: &str, user_name: &str, prefers_formal: bool, topics: Vec<String>) {
        let memory = Arc::new(Mutex::new(MemoryRouter::new()));
        let emotion_state = EmotionState::neutral();
        let user_profile = UserProfile {
            name: user_name.to_string(),
            prefers_formal,
            favorite_topics: topics,
        };
        let session = LLMSession {
            id: session_id.to_string(),
            context_tokens: Vec::new(),
            memory,
            emotion_state,
            last_accessed: Instant::now(),
            intents: Vec::new(),
            user_profile,
        };
        self.sessions.insert(session_id.to_string(), session);
    }

    /// Returns Ok<(response, duration_in_ms)>
    pub fn process_input(&mut self, session_id: &str, input: &str) -> Result<(String, u128), String> {
        let start_time = Instant::now();

        let session = self.sessions.get_mut(session_id)
            .ok_or_else(|| format!("Session {} not found", session_id))?;

        let tokens = self.tokenizer.tokenize(input);
        session.context_tokens.extend(tokens.clone());
        session.last_accessed = Instant::now();

        {
            let mut mem = session.memory.lock().unwrap();
            mem.integrate_tokens(&tokens);
        }

        session.emotion_state = EmotionState::from_text(input);
        let intent = Intent::from_text(input);
        session.intents.push(intent.clone());

        let response_tokens = self.generate_response(&session);
        let response = self.tokenizer.detokenize(&response_tokens);

        let duration_ms = start_time.elapsed().as_millis();
        Ok((response, duration_ms))
    }

    fn generate_response(&self, session: &LLMSession) -> Vec<Token> {
        let context = &session.context_tokens;
        let slice_len = std::cmp::min(10, context.len());
        let mut response_tokens = context[context.len() - slice_len..].to_vec();

        // Personalization based on emotional tone and user preferences
        let greeting = if session.user_profile.prefers_formal {
            format!("Hello {}, how may I assist you today?", session.user_profile.name)
        } else {
            format!("Hey {}, what's up?", session.user_profile.name)
        };

        let greeting_tokens = self.tokenizer.tokenize(&greeting);
        let mood_tokens = self.tokenizer.tokenize(&format!("I'm sensing you're feeling {:?}.", session.emotion_state));

        // Build response
        let mut result = greeting_tokens;
        result.extend(mood_tokens);
        result.extend(response_tokens.iter().rev()); // reversed context
        result.push(self.tokenizer.end_token());

        result
    }

    pub fn cleanup_sessions(&mut self, timeout_secs: u64) {
        let now = Instant::now();
        self.sessions.retain(|_, session| {
            now.duration_since(session.last_accessed) < Duration::from_secs(timeout_secs)
        });
    }
}
