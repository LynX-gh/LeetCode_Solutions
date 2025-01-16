use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut char_map = HashMap::new();

        for c in s.chars()
        {
            *char_map.entry(c).or_insert(0) += 1;
        }

        for c in t.chars()
        {
            *char_map.entry(c).or_insert(0) -= 1;
        }

        for &num in char_map.values()
        {
            if num != 0
            {
                return false;
            }
        }
        true
    }
}