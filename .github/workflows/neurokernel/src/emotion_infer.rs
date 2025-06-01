use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Emotion {
    Neutral,
    Happy,
    Sad,
    Angry,
    Curious,
    Surprised,
    Anxious,
    Confident,
}

pub struct EmotionInfer {
    keyword_map: HashMap<&'static str, Emotion>,
}

impl EmotionInfer {
    pub fn new() -> Self {
        let mut keyword_map = HashMap::new();

        keyword_map.insert("awesome", Emotion::Happy);
        keyword_map.insert("great", Emotion::Happy);
        keyword_map.insert("sad", Emotion::Sad);
        keyword_map.insert("unfortunately", Emotion::Sad);
        keyword_map.insert("angry", Emotion::Angry);
        keyword_map.insert("furious", Emotion::Angry);
        keyword_map.insert("why", Emotion::Curious);
        keyword_map.insert("how", Emotion::Curious);
        keyword_map.insert("wow", Emotion::Surprised);
        keyword_map.insert("amazing", Emotion::Surprised);
        keyword_map.insert("nervous", Emotion::Anxious);
        keyword_map.insert("worried", Emotion::Anxious);
        keyword_map.insert("sure", Emotion::Confident);
        keyword_map.insert("definitely", Emotion::Confident);

        EmotionInfer { keyword_map }
    }

    pub fn infer(&self, input: &str) -> Emotion {
        let lowercase_input = input.to_lowercase();
        let mut scores = HashMap::new();

        for (word, emotion) in self.keyword_map.iter() {
            if lowercase_input.contains(word) {
                *scores.entry(emotion.clone()).or_insert(0) += 1;
            }
        }

        scores
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(emotion, _)| emotion)
            .unwrap_or(Emotion::Neutral)
    }
}
