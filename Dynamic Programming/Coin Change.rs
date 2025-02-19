impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![0; amount as usize + 1];
        dp[0] = 0;

        for i in 1..=amount as usize {
            dp[i] = i32::MAX;

            for &coin in coins.iter() {
                let last_min = i - coin as usize;

                if last_min < dp.len() && dp[last_min] != i32::MAX {
                    dp[i] = dp[i].min(dp[last_min] + 1);
                }
            }
        }

        if dp[dp.len()-1] == i32::MAX { -1 } else { dp[dp.len()-1] }
    }
}