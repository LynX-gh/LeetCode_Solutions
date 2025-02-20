// Backtrack O(2^n)
use std::collections::HashSet;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut n = nums[0].len();
        let mut seen = HashSet::from_iter(nums);
        let mut res = String::new();
        
        Self::backtrack(&mut res, &seen, &n);
        res
    }

    fn backtrack(res: &mut String, seen: &HashSet<String>, target: &usize) -> bool {
        if res.len() == *target {
            return !seen.contains(res);
        }

        res.push('0');
        if Self::backtrack(res, seen, target) {
            return true;
        }
        res.pop();

        res.push('1');
        if Self::backtrack(res, seen, target) {
            return true;
        }
        res.pop();

        false
    }
}