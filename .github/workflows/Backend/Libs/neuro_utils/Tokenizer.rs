// libs/neuro_utils/src/tokenizer.rs

use std::collections::HashMap;

/// Tokenizer struct that maps strings to tokens and vice versa.
pub struct Tokenizer {
    vocab: HashMap<String, usize>,
    inv_vocab: HashMap<usize, String>,
}

impl Tokenizer {
    /// Create a new Tokenizer from a vocabulary list
    pub fn new(vocab_list: Vec<String>) -> Self {
        let mut vocab = HashMap::new();
        let mut inv_vocab = HashMap::new();
        for (idx, token) in vocab_list.into_iter().enumerate() {
            vocab.insert(token.clone(), idx);
            inv_vocab.insert(idx, token);
        }
        Tokenizer { vocab, inv_vocab }
    }

    /// Encode a string into a vector of tokens (usize)
    pub fn encode(&self, input: &str) -> Vec<usize> {
        input
            .split_whitespace()
            .map(|word| *self.vocab.get(word).unwrap_or(&0)) // 0 for unknown token
            .collect()
    }

    /// Decode a vector of tokens back into a string
    pub fn decode(&self, tokens: &[usize]) -> String {
        tokens
            .iter()
            .map(|token| self.inv_vocab.get(token).unwrap_or(&"<UNK>".to_string()).clone())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer_encode_decode() {
        let vocab = vec![
            "hello".to_string(),
            "world".to_string(),
            "this".to_string(),
            "is".to_string(),
            "test".to_string(),
        ];
        let tokenizer = Tokenizer::new(vocab);

        let text = "hello world this is unknown";
        let tokens = tokenizer.encode(text);
        assert_eq!(tokens, vec![0, 1, 2, 3, 0]); // 'unknown' maps to 0

        let decoded = tokenizer.decode(&tokens);
        assert_eq!(decoded, "hello world this is hello");
    }
}
