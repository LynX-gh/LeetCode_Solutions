use std::collections::VecDeque;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut q = VecDeque::new();
        let mut res = 0;
        let mut curr = 0;
        let mut start = 0;

        for i in 0..nums.len() {
            if nums[i] == 0 { 
                q.push_back(i);
                if q.len() == k as usize + 1 {
                    start = q.pop_front().unwrap() + 1;
                }
            }
            res = res.max(i-start+1);
        }
        res as i32
    }
}