use std::collections::HashSet;

// O(n)
impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let v: Vec<i32> = nums.iter().cloned().collect::<HashSet<_>>()
            .into_iter()
            .filter(|&x| x > 0)
            .collect();

        if v.is_empty() {
            return *nums.iter().max().unwrap();
        }

        v.iter().sum()
    }
}

// O(n^2)
use std::collections::HashSet;

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut res = i32::MIN;
        let mut seen = HashSet::new();

        for i in 0..nums.len() {
            let mut cur = 0;
            seen.clear();
            for j in i..nums.len() {
                if seen.insert(nums[j]) && (cur == 0 || nums[j] > 0) {
                    cur += nums[j];
                    res = res.max(cur);
                }
            }
        }
        res
    }
}