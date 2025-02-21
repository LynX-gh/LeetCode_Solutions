impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        let mut dp = vec![vec![0; n]; m];
        if obstacle_grid[m-1][n-1] != 1 {
            dp[m-1][n-1] = 1;
        }

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if i - 1 < m && obstacle_grid[i-1][j] != 1 {
                    dp[i-1][j] += dp[i][j];
                }
                if j - 1 < n && obstacle_grid[i][j-1] != 1 {
                    dp[i][j-1] += dp[i][j];
                }
            }
        }

        dp[0][0]
    }
}