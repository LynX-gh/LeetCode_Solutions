use std::collections::{HashSet, HashMap};

// DP
use std::collections::HashSet;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut dp = vec![false; s.len() + 1];
        let mut set: HashSet<String> = HashSet::from_iter(word_dict);
        dp[s.len()] = true;

        for i in (0..s.len()).rev() {
            for word in set.iter() {
                if i + word.len() <= s.len() && &s[i..i+word.len()] == word && dp[i+word.len()] == true {
                    dp[i] = true;
                    break;
                }
            }
        }
        dp[0]
    }
}

// Backtrack
impl Solution {
    pub fn word_break(s: String, mut word_dict: Vec<String>) -> bool {
        let mut set = HashSet::from_iter(word_dict);
        let mut map = HashMap::new();
        Self::backtrack(&s, s.len(), &set, &mut map)
    }

    fn backtrack(s: &String, index: usize, set: &HashSet<String>, memo: &mut HashMap<usize, bool>) -> bool {
        if index == 0 {
            return true;
        }

        if let Some(res) = memo.get(&index) {
            return *res;
        }

        for word in set.iter() {
            if index-word.len() < s.len() && &s[index-word.len()..index] == word {
                let res = Self::backtrack(s, index-word.len(), set, memo);
                memo.insert(index-word.len(), res);
                if res {
                    return true;
                }
            }
        }
        false
    }
}