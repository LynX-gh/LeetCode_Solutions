impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut total: i32 = nums.iter().sum();
        if total % 2 != 0 {
            return false;
        }

        let mut dp = vec![false; (total/2+1) as usize];
        dp[0] = true;

        for i in 1..nums.len() {
            for j in (0..dp.len()).rev() {
                if (j - nums[i-1] as usize) < dp.len() {
                    dp[j] = dp[j - nums[i-1] as usize] || dp[j];
                }
            }
        }
        dp[(total/2) as usize]
    }
}
