// Bottom UP
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n]; m];
        dp[m-1][n-1] = 1;

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if i-1 < m {
                    dp[i-1][j] += dp[i][j];
                }
                if j-1 < n {
                    dp[i][j-1] += dp[i][j];
                }
            }
        }
        println!("{:?}", dp);
        dp[0][0]
    }
}

// Top Down
impl Solution {
    pub fn unique_paths(mut m: i32, mut n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut memo = vec![vec![0; n]; m];
        memo[0][0] = 1;

        for i in 0..m {
            for j in 0..n {
                if i+1 < m {
                    memo[i+1][j] += memo[i][j];
                }
                if j+1 < n {
                    memo[i][j+1] += memo[i][j];
                }
            }
        }
        memo[m-1][n-1]
    }
}