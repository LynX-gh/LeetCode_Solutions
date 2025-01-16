use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut char_map_s = HashMap::with_capacity(s.len());
        let s_byte = s.as_bytes();
        let t_byte = t.as_bytes();

        for i in 0..s_byte.len() {
            let mut ctr_s = char_map_s.entry(&s_byte[i]).or_insert(&t_byte[i]);
            if **ctr_s != t_byte[i] {
                return false;
            }
        }
        char_map_s.keys().collect::<HashSet<_>>().len() == char_map_s.values().collect::<HashSet<_>>().len()
    }
}