impl Solution {
    pub fn max_profit(mut m: i32, prices: Vec<i32>) -> i32 {
        let k = m as usize;
        if k == 0 {
            return 0;
        }

        let mut dp = vec![vec![0, 0]; k as usize + 1];
        for i in 0..=k {
            dp[i][0] = i32::MAX;
        }

        for price in prices {
            for i in 1..=k {
                dp[i][0] = dp[i][0].min(price - dp[i-1][1]);
                dp[i][1] = dp[i][1].max(price - dp[i][0]);
            }
        }

        dp[k][1]
    }
}