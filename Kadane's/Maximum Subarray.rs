impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut last_sum = 0;
        let mut res = nums[0];

        for num in nums {
            last_sum += num;
            res = res.max(last_sum);
            if last_sum < 1 {
                last_sum = 0;
            }
        }
        res
    }
}