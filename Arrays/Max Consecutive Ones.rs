use std::cmp;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut ctr = 0;
        for num in nums {
            if num == 1 {
                ctr += 1;
            }
            if num == 0 {
                ctr = 0;
            }
            res = cmp::max(ctr, res);
        }
        res
    }
}