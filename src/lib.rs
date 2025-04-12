use std::collections::HashMap;

pub struct BpeTokenizer {
    merges: HashMap<(String, String), String>,
    vocab: HashMap<String, i32>,
    reverse_vocab: HashMap<i32, String>,
    next_id: i32,
}

impl BpeTokenizer {
    pub fn new() -> BpeTokenizer {
        BpeTokenizer { 
            merges: HashMap::new(),
            vocab: HashMap::new(),
            reverse_vocab: HashMap::new(),
            next_id: 0,
       }
    }

    // Initialize vocab size with unique characters from the provided inputs (words)
    pub fn initialize_vocab(&mut self, words: &[String]) {
        for word in words {
            for character in word.chars() {
                let char_str: String = character.to_string();

                if !self.vocab.contains_key(&char_str) {
                    let new_id: i32 = self.next_id;
                    self.vocab.insert(char_str.clone(), new_id);
                    self.reverse_vocab.insert(new_id, char_str);
                    self.next_id += 1;
                }
                // If char is already in the vocab, skip
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tokenizer_is_empty() {
        let tokenizer: BpeTokenizer = BpeTokenizer::new();

        assert!(tokenizer.merges.is_empty(), "Merges should be empty initially.");
        assert!(tokenizer.vocab.is_empty(), "Vocabulary should be empty initially.");
        assert!(tokenizer.reverse_vocab.is_empty(), "Reverse vocabulary should be empty initially.");
        assert_eq!(tokenizer.next_id, 0, "Initial next_id should be 0.")
    }

    #[test]
    fn test_initialize_vocab() {
        let mut tokenizer: BpeTokenizer = BpeTokenizer::new();
        let words: Vec<String> = vec![
            "Hello".to_string(),
            "World".to_string(),
            "hello".to_string(),
            "World".to_string(), // uniqueness check
            "world".to_string(),
        ];

        tokenizer.initialize_vocab(&words);

        let expected_unique_chars= 9;

        assert_eq!(
            tokenizer.vocab.len(),
            expected_unique_chars,
            "Vocabulary size should match the number of unique characters."
        );
        assert_eq!(
            tokenizer.reverse_vocab.len(),
            expected_unique_chars,
            "Reverse vocabulary size should match the number of unique characters."
        );
        assert_eq!(
            tokenizer.next_id,
            expected_unique_chars as i32,
            "next_id should be equal to the number of unique characters added."
        );

        assert!(tokenizer.vocab.contains_key("h"), "Vocab should contain 'h'.");
        assert!(tokenizer.vocab.contains_key("e"), "Vocab should contain 'e'.");
        assert!(tokenizer.vocab.contains_key("l"), "Vocab should contain 'l'.");
        assert!(tokenizer.vocab.contains_key("o"), "Vocab should contain 'o'.");
        assert!(tokenizer.vocab.contains_key("w"), "Vocab should contain 'w'.");
        assert!(tokenizer.vocab.contains_key("r"), "Vocab should contain 'r'.");
        assert!(tokenizer.vocab.contains_key("d"), "Vocab should contain 'd'.");

        assert!(tokenizer.reverse_vocab.contains_key(&0), "Reverse vocab should contain ID 0.");
         for i in 0..tokenizer.next_id {
            assert!(tokenizer.reverse_vocab.contains_key(&i), "Reverse vocab should contain ID {}.", i);
        }
    }
}