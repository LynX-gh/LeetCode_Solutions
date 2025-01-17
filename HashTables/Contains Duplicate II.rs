use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut seen = HashMap::new();
        let k = k as usize;

        for i in 0..nums.len() {
            match seen.get(&nums[i]) {
                Some(index) if i.abs_diff(*index) <= k => return true,
                _ => seen.insert(&nums[i], i),
            };
        }
        false
    }
}