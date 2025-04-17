use std::collections::HashMap;

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut idx_map = HashMap::new();
        let mut res = 0;

        for i in 0..nums.len() {
            let indexes = idx_map.entry(nums[i]).and_modify(|idxs: &mut Vec<usize>| idxs.push(i)).or_insert(vec![i]);
            for &idx in indexes.iter() {
                if idx != i && (idx * i) as i32 % k == 0 {
                    res += 1;
                }
            }
        }
        res
    }
}
