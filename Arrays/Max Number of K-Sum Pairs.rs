use std::collections::HashMap;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            map.entry(nums[i]).and_modify(|ctr| *ctr+=1).or_insert(1);
        }

        for i in 0..nums.len() {
            if let Some(&ctr) = map.get(&(k - nums[i])) {
                if (nums[i] == k - nums[i] && ctr > 1) || (nums[i] != k - nums[i] && *map.get(&nums[i]).unwrap() > 0 && ctr > 0) {
                    res += 1;
                    map.entry(nums[i]).and_modify(|ctr| *ctr -= 1);
                    map.entry(k - nums[i]).and_modify(|ctr| *ctr -= 1);
                }
            }
        }
        res
    }
}