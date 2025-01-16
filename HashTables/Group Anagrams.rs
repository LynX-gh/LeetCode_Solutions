use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();
        
        for s in strs.iter() {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort_unstable();
            let key = chars.into_iter().collect::<String>();
            
            map.entry(key).or_insert(Vec::new()).push(s.clone());
        }
        
        map.into_values().collect()
    }
}