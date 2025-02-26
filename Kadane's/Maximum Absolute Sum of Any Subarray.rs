impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut kadane_max = 0;
        let mut kadane_min = 0;
        let mut curr_max = 0;
        let mut curr_min = 0;

        for i in 0..nums.len() {
            curr_max = 0.max(curr_max + nums[i]);
            kadane_max = kadane_max.max(curr_max);

            curr_min = 0.min(curr_min + nums[i]);
            kadane_min = kadane_min.min(curr_min);
        }
        kadane_min.abs().max(kadane_max)
    }
}