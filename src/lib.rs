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
}