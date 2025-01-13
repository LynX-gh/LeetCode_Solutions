use std::cmp::max;
use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set = HashSet::new();
        let mut sptr = 0;
        let mut eptr = 0;
        let mut res = 0;
        let mut curr_max = 0;
        
        let s_byte = s.as_bytes();

        while eptr < s.len() {
            if !set.contains(&s_byte[eptr]) {
                set.insert(s_byte[eptr]);
                curr_max += 1;
                eptr += 1;
            }
            else {
                res = max(res, curr_max);
                while s_byte[sptr] != s_byte[eptr] {
                    set.remove(&s_byte[sptr]);
                    sptr += 1;
                    curr_max -= 1;
                }
                set.remove(&s_byte[sptr]);
                sptr += 1;
                curr_max -= 1;
            }
        }
        max(res, curr_max)
    }
}