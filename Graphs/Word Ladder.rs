use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut res = 0;
        let mut word_set:HashSet<_> = word_list.clone().into_iter().collect();
        let mut bfs = VecDeque::new();

        if let None = word_set.get(&end_word) {
            return 0;
        }
        
        bfs.push_back((begin_word.clone(), 0));
        word_set.remove(&begin_word);

        while let Some((q_word, moves)) = bfs.pop_front() {
            if q_word == end_word {
                return moves+1;
            }

            for i in 0..q_word.len() {
                let mut word_chars: Vec<char> = q_word.chars().collect();
                let temp = word_chars[i];
                for c in b'a'..=b'z' {
                    word_chars[i] = c as char;
                    let new_word: String = word_chars.iter().collect();

                    if word_set.contains(&new_word) {
                        word_set.remove(&new_word);
                        bfs.push_back((new_word, moves + 1));
                    }
                }
                word_chars[i] = temp;
            }
        }

        0
    }
}