use std::collections::HashMap;

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let mut freq_map = HashMap::new();
        let mut res = 0;
        let mut s = 0;

        for i in 0..nums.len() {
            freq_map.entry(nums[i]).and_modify(|ctr| *ctr += 1).or_insert(1);
        }
        let u = freq_map.len();
        freq_map.clear();

        for i in 0..nums.len() {
            freq_map.entry(nums[i]).and_modify(|ctr| *ctr += 1).or_insert(1);
            while freq_map.len() == u {
                res += nums.len() - i;
                let mut ctr = freq_map.entry(nums[s]).or_insert(0);
                *ctr -= 1;
                if *ctr == 0 {
                    freq_map.remove(&nums[s]);
                }
                s += 1;
            }
        }
        res as i32
    }
}