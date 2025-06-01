// neurokernel/src/emotion_infer.rs

use std::collections::HashMap;

/// Enum representing different emotion categories
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Emotion {
    Joy,
    Sadness,
    Anger,
    Fear,
    Surprise,
    Disgust,
    Neutral,
}

/// Struct to hold emotion inference model parameters
pub struct EmotionInferModel {
    /// Lexicon mapping words to emotion scores
    lexicon: HashMap<String, HashMap<Emotion, f32>>,
}

impl EmotionInferModel {
    /// Initialize a new EmotionInferModel with given lexicon
    pub fn new(lexicon: HashMap<String, HashMap<Emotion, f32>>) -> Self {
        EmotionInferModel { lexicon }
    }

    /// Infer emotion scores from a text input
    pub fn infer_emotions(&self, text: &str) -> HashMap<Emotion, f32> {
        let mut emotion_scores: HashMap<Emotion, f32> = HashMap::new();

        // Initialize all emotion scores to zero
        for emotion in [
            Emotion::Joy,
            Emotion::Sadness,
            Emotion::Anger,
            Emotion::Fear,
            Emotion::Surprise,
            Emotion::Disgust,
            Emotion::Neutral,
        ] {
            emotion_scores.insert(emotion.clone(), 0.0);
        }

        // Tokenize text simply by whitespace
        for word in text.split_whitespace() {
            let word_lower = word.to_lowercase();
            if let Some(word_emotions) = self.lexicon.get(&word_lower) {
                for (emotion, score) in word_emotions.iter() {
                    *emotion_scores.entry(emotion.clone()).or_insert(0.0) += score;
                }
            }
        }

        // Normalize scores by total to get relative weights
        let total_score: f32 = emotion_scores.values().sum();
        if total_score > 0.0 {
            for score in emotion_scores.values_mut() {
                *score /= total_score;
            }
        } else {
            // If no scores, set Neutral to 1.0
            emotion_scores.insert(Emotion::Neutral, 1.0);
        }

        emotion_scores
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infer_emotions_basic() {
        let mut lexicon = HashMap::new();

        lexicon.insert(
            "happy".to_string(),
            [(Emotion::Joy, 1.0)].iter().cloned().collect(),
        );
        lexicon.insert(
            "sad".to_string(),
            [(Emotion::Sadness, 1.0)].iter().cloned().collect(),
        );
        lexicon.insert(
            "angry".to_string(),
            [(Emotion::Anger, 1.0)].iter().cloned().collect(),
        );

        let model = EmotionInferModel::new(lexicon);

        let text = "I am happy but also a bit sad";
        let emotions = model.infer_emotions(text);

        assert!(emotions[&Emotion::Joy] > 0.4);
        assert!(emotions[&Emotion::Sadness] > 0.4);
        assert!(emotions[&Emotion::Anger] == 0.0);
    }
}
