// DP O(n^2)
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut dp = vec![0; nums.len()];

        for i in (0..nums.len()).rev() {
            dp[i] = 1;

            for j in i+1..nums.len() {
                if nums[j] > nums[i] && dp[j] + 1 > dp[i]{
                    dp[i] = dp[j] + 1
                }
            }
            res = res.max(dp[i]);
        }
        res
    }
}