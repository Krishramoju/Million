use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token(pub u32); // Could represent a byte-pair index or vocab ID

pub struct Tokenizer {
    vocab: HashMap<String, u32>,
    inverse_vocab: HashMap<u32, String>,
    special_tokens: HashMap<String, u32>,
}

impl Tokenizer {
    pub fn new() -> Self {
        // Simple static vocab for now
        let mut vocab = HashMap::new();
        let mut inverse_vocab = HashMap::new();
        let mut special_tokens = HashMap::new();

        // Populate with basic examples
        let tokens = vec![
            ("[PAD]", 0),
            ("[UNK]", 1),
            ("[CLS]", 2),
            ("[SEP]", 3),
            ("[END]", 4),
            ("hello", 100),
            ("world", 101),
            (",", 102),
            ("how", 103),
            ("are", 104),
            ("you", 105),
            ("?", 106),
        ];

        for (word, id) in tokens {
            vocab.insert(word.to_string(), id);
            inverse_vocab.insert(id, word.to_string());
        }

        special_tokens.insert("[PAD]".to_string(), 0);
        special_tokens.insert("[UNK]".to_string(), 1);
        special_tokens.insert("[CLS]".to_string(), 2);
        special_tokens.insert("[SEP]".to_string(), 3);
        special_tokens.insert("[END]".to_string(), 4);

        Tokenizer {
            vocab,
            inverse_vocab,
            special_tokens,
        }
    }

    pub fn tokenize(&self, text: &str) -> Vec<Token> {
        text.split_whitespace()
            .map(|word| {
                let cleaned = word.to_lowercase();
                let id = self
                    .vocab
                    .get(&cleaned)
                    .cloned()
                    .unwrap_or(self.special_tokens["[UNK]"]);
                Token(id)
            })
            .collect()
    }

    pub fn detokenize(&self, tokens: &[Token]) -> String {
        tokens
            .iter()
            .map(|token| {
                self.inverse_vocab
                    .get(&token.0)
                    .cloned()
                    .unwrap_or("[UNK]".to_string())
            })
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn end_token(&self) -> Token {
        Token(self.special_tokens["[END]"])
    }

    pub fn pad_token(&self) -> Token {
        Token(self.special_tokens["[PAD]"])
    }

    pub fn unk_token(&self) -> Token {
        Token(self.special_tokens["[UNK]"])
    }
}
