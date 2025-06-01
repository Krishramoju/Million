use rand::{seq::SliceRandom, Rng};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct SimulatedUser {
    pub user_id: String,
    pub name: String,
    pub personality: PersonalityProfile,
    pub memory_log: Vec<UserInteraction>,
}

#[derive(Debug, Clone)]
pub struct PersonalityProfile {
    pub curiosity_level: f32,     // 0.0 to 1.0
    pub emotional_sensitivity: f32,
    pub assertiveness: f32,
    pub creativity: f32,
}

#[derive(Debug, Clone)]
pub struct UserInteraction {
    pub timestamp: DateTime<Utc>,
    pub message: String,
    pub detected_emotion: String,
    pub system_response: String,
}

impl SimulatedUser {
    pub fn new(user_id: &str, name: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            name: name.to_string(),
            personality: PersonalityProfile {
                curiosity_level: 0.6,
                emotional_sensitivity: 0.5,
                assertiveness: 0.4,
                creativity: 0.7,
            },
            memory_log: vec![],
        }
    }

    pub fn generate_interaction(&mut self) -> UserInteraction {
        let prompts = vec![
            "What's the weather like tomorrow?",
            "Tell me a story about space dragons.",
            "How do I start a tech startup?",
            "I'm feeling overwhelmed today.",
            "What can I cook with eggs and cheese?",
            "Why do I keep procrastinating?",
        ];

        let message = prompts.choose(&mut rand::thread_rng()).unwrap().to_string();
        let emotion = Self::simulate_emotion(&message);
        let response = format!("LLM: Responding to '{}'", message);

        let interaction = UserInteraction {
            timestamp: Utc::now(),
            message: message.clone(),
            detected_emotion: emotion,
            system_response: response.clone(),
        };

        self.memory_log.push(interaction.clone());
        interaction
    }

    fn simulate_emotion(input: &str) -> String {
        if input.contains("overwhelmed") {
            "Anxious".to_string()
        } else if input.contains("space dragons") {
            "Curious".to_string()
        } else if input.contains("procrastinating") {
            "Sad".to_string()
        } else {
            "Neutral".to_string()
        }
    }

    pub fn replay_memory(&self) {
        for interaction in &self.memory_log {
            println!(
                "[{}] {} | Emotion: {} | Response: {}",
                interaction.timestamp, interaction.message, interaction.detected_emotion, interaction.system_response
            );
        }
    }
}
