// use std::collections::VecDeque;

impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let mut s = Vec::new();

        for i in 0..nums.len() {
            if s.is_empty() || nums[*s.last().unwrap()] > nums[i] {
                s.push(i)
            }
        }

        let mut res = 0;
        for i in (0..nums.len()).rev() {
            while !s.is_empty() && nums[i] >= nums[*s.last().unwrap()] {
                res = res.max(i - s.pop().unwrap());
            }
        }
        res as i32
    }
}