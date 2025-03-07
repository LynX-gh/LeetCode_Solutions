use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut freq = HashMap::new();
        let mut seen = HashSet::new();

        for i in arr {
            freq.entry(i).and_modify(|ctr| *ctr += 1).or_insert(1);
        }

        for &val in freq.values() {
            if !seen.insert(val) {
                return false;
            }
        }
        true
    }
}