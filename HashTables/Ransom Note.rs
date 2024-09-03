use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut map = HashMap::new();
        for ch in magazine.chars() {
            map.entry(ch).and_modify(|ctr| *ctr += 1).or_insert(1);
        }
        for ch in ransom_note.chars() {
            match map.get(&ch) {
                Some(x) if *x > 0 => map.entry(ch).and_modify(|ctr| *ctr -= 1),
                Some(x) if *x == 0 => return false,
                _ => return false, 
            };
        }
        true
    }
}