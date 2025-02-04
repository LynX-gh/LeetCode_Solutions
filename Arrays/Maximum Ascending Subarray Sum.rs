use std::cmp;

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut res = nums[0];
        let mut cur_sum = nums[0];

        for i in 1..nums.len() {
            if nums[i] > nums[i-1] {
                cur_sum += nums[i];
            } else {
                res = cmp::max(res, cur_sum);
                cur_sum = nums[i];
            }
        }
        cmp::max(res, cur_sum)
    }
}