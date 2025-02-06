use std::collections::HashMap;

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut prod_map = HashMap::new();

        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                let mut ctr = prod_map.entry(nums[i]*nums[j]).or_insert(0);
                res += 8 * *ctr;
                *ctr += 1;
            }
        }

        res
    }
}