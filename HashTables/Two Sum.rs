use std::collections::HashMap;

// O(n)
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 2 {
            return vec![0,1];
        }

        let mut num_map = HashMap::with_capacity(nums.len());

        for i in 0..nums.len() {
            match num_map.get(&(target - nums[i])) {
                Some(x) if *x != i => return vec![i as i32, *x as i32],
                _ => num_map.insert(nums[i], i),
            };
        };
        vec![0, 0]
    }
}


// O(2n)
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 2 {
            return vec![0,1];
        }

        let mut num_map = HashMap::with_capacity(nums.len());

        for i in 0..nums.len() {
            num_map.insert(nums[i], i);
        }

        for i in 0..nums.len() {
            match num_map.get(&(target - nums[i])) {
                Some(x) if *x != i => return vec![i as i32, *x as i32],
                _ => ()
            }
        };
        vec![0, 0]
    }
}