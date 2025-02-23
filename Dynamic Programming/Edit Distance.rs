// DP
impl Solution {
    pub fn min_distance(w1: String, w2: String) -> i32 {
        let word1 = w1.as_bytes();
        let word2 = w2.as_bytes();
        let l1 = word1.len();
        let l2 = word2.len();
        let mut dp = vec![vec![usize::MAX; word2.len()+1]; word1.len()+1];

        for i in 0..l1+1 {
            dp[i][l2] = l1 - i;
        }
        for j in 0..l2+1 {
            dp[l1][j] = l2 - j;
        }

        for i in (0..l1).rev() {
            for j in (0..l2).rev() {
                if word1[i] == word2[j] {
                    dp[i][j] = dp[i+1][j+1];
                } else {
                    dp[i][j] = dp[i+1][j+1].min(dp[i+1][j].min(dp[i][j+1])) + 1;
                }
            }
        }
        dp[0][0] as i32
    }
}