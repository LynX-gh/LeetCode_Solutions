impl Solution {
    pub fn maximal_square(mut matrix: Vec<Vec<char>>) -> i32 {
        let mut res = 0;
        let mut dp = vec![vec![0; matrix[0].len()+1]; matrix.len()+1];
        for i in (0..matrix.len()).rev() {
            for j in (0..matrix[0].len()).rev() {
                if matrix[i][j] != '0' {
                    dp[i][j] = dp[i+1][j].min(dp[i][j+1].min(dp[i+1][j+1])) + 1;
                    res = res.max(dp[i][j]);
                }
            }
        }
        res * res
    }
}