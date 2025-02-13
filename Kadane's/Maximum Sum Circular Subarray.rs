impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut max_sum = nums[0];
        let mut last_max = 0;
        let mut min_sum = nums[0];
        let mut last_min = 0;
        let mut total = 0;

        for num in nums.into_iter() {
            last_max += num;
            last_min += num;

            last_max = last_max.max(num);
            last_min = last_min.min(num);

            max_sum = max_sum.max(last_max);
            min_sum = min_sum.min(last_min);
            total += num;
        }

        if max_sum > 0 {
            max_sum.max(total - min_sum)
        } else {
            max_sum
        }
    }
}