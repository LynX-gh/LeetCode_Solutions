impl Solution {
    pub fn maximum_possible_size(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut last_max = nums[0];
        for i in 0..nums.len() {
            if nums[i] >= last_max {
                res += 1;
                last_max = nums[i];
            }
        }
        res
    }
}
