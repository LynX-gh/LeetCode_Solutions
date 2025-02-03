use std::cmp;

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut cur_inc = 1;
        let mut cur_dec = 1;

        for i in 1..nums.len() {
            match nums[i].cmp(&nums[i-1]) {
                cmp::Ordering::Less => {
                    cur_dec += 1;;
                    res = cmp::max(res, cur_inc);
                    cur_inc = 1;
                },
                cmp::Ordering::Greater => {
                    cur_inc += 1;
                    res = cmp::max(res, cur_dec);
                    cur_dec = 1;
                },
                cmp::Ordering::Equal => {
                    res = cmp::max(res, cur_dec);
                    cur_dec = 1;
                    res = cmp::max(res, cur_inc);
                    cur_inc = 1;
                }
            }
        }
        res.max(cur_inc.max(cur_dec))
    }
}