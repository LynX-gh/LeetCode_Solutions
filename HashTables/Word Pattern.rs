use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut pattern_map = HashMap::with_capacity(pattern.len());
        let mut seen_words = HashSet::with_capacity(pattern.len());

        let pattern_byte = pattern.as_bytes();
        let s_words = s.split_whitespace().collect::<Vec<_>>();

        if pattern_byte.len() != s_words.len() {
            return false;
        }

        for i in 0..pattern_byte.len() {
            if let Some(mapped_word) = pattern_map.get(&pattern_byte[i]) {
                if *mapped_word != s_words[i] {
                    return false;
                }
            } else {
                if seen_words.contains(s_words[i]) {
                    return false;
                }
                pattern_map.insert(pattern_byte[i], s_words[i]);
                seen_words.insert(s_words[i]);
            }
        }
        true
    }
}