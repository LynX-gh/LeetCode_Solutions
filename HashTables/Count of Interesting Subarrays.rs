use std::collections::HashMap;
impl Solution {
    pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
        let mut cnt = HashMap::new();
        cnt.insert(0, 1);
        let mut res = 0i64;
        let mut prefix = 0;
        for &num in &nums {
            if num % modulo == k {
                prefix += 1;
            }
            let target = (prefix - k + modulo) % modulo;
            res += *cnt.get(&(target)).unwrap_or(&0) as i64;
            *cnt.entry(prefix % modulo).or_insert(0) += 1;
        }
        res
    }
}