use std::collections::HashSet;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = HashSet::new();
        for &num in &nums {
            if num > k {
                res.insert(num);
            } else if num < k {
                return -1;
            }
        }
        res.len() as i32
    }
}
