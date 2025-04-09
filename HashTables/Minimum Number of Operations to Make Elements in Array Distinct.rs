use std::collections::HashSet;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();

        for i in (0..nums.len()).rev() {
            if !set.insert(nums[i]) {
                return ((i as i32 + 3) / 3)
            }
        }
        0
    }
}
