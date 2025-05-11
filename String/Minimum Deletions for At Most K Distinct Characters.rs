use std::collections::HashMap;

impl Solution {
    pub fn min_deletion(s: String, k: i32) -> i32 {
        let mut freq = HashMap::new();
        for ch in s.chars() {
            freq.entry(ch).and_modify(|ctr| *ctr += 1).or_insert(1);
        }

        if freq.keys().len() <= k as usize {
            0
        } else {
            let mut counts: Vec<_> = freq.values().collect();
            counts.sort_unstable();
            let mut n = freq.keys().len() as i32;
            let mut res = 0;
            let mut i = 0;
            while n > k {
                res += counts[i];
                i += 1;
                n -= 1;
            }
            res
        }
    }
}