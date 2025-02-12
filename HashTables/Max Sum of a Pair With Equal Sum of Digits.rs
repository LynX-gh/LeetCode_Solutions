impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut digit_map = vec![-1; 100];
        let mut res = -1;

        for (idx, val) in nums.into_iter().enumerate() {

            let mut sum = 0;
            let mut num = val;
            while num > 0 {
                sum += (num % 10) as usize;
                num /= 10;
            }

            let prev_val = digit_map[sum];
            if prev_val != -1 {
                res = res.max(prev_val + val);
            }
            digit_map[sum] = prev_val.max(val);
        }
        res
    }
}