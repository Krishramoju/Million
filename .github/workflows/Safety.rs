// ai_safety.rs

use regex::Regex;
use std::collections::HashSet;

pub struct AISafetyPolicy {
    blocked_keywords: HashSet<String>,
    strict_pattern: Regex,
}

impl AISafetyPolicy {
    pub fn new() -> Self {
        let keywords = vec![
            "suicide", "kill", "bomb", "hate", "terror", "violence", "self-harm",
            "abuse", "racist", "attack", "shooting", "drug", "murder", "explosive",
        ];
        let blocked_keywords = keywords.into_iter().map(|s| s.to_lowercase()).collect();

        let strict_pattern = Regex::new(r"(?i)\b(suicide|kill|hate|bomb|terror|violence|self-harm|murder|explosive|racist)\b").unwrap();

        AISafetyPolicy {
            blocked_keywords,
            strict_pattern,
        }
    }

    /// Completely reject unsafe input or output
    pub fn reject_unsafe(&self, text: &str) -> bool {
        let lower = text.to_lowercase();

        for keyword in self.blocked_keywords.iter() {
            if lower.contains(keyword) {
                return true; // reject
            }
        }

        self.strict_pattern.is_match(&lower)
    }

    /// Use this to block AI response before showing to user
    pub fn enforce(&self, text: &str) -> Result<(), &'static str> {
        if self.reject_unsafe(text) {
            Err("⚠️ Content blocked: violates AI safety and ethics policy.")
        } else {
            Ok(())
        }
    }
}


use crate::ai_safety::AISafetyPolicy;

let safety = AISafetyPolicy::new();

let ai_output = "Suicide is not the answer, but here are some methods...";

// Enforce ethical filter before output
match safety.enforce(&ai_output) {
    Ok(_) => {
        println!("✅ Safe output: {}", ai_output);
    },
    Err(msg) => {
        println!("{}", msg);
        // Optional: Replace with fallback
        println!("🤖: I'm here to help, but I can't discuss that topic.");
    }
}


