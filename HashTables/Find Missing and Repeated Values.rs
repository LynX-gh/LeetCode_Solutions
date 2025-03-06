impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let mut seen = vec![0; (n * n) + 1];
        let mut res = vec![0, 0];

        for i in 0..n {
            for j in 0..n {
                seen[grid[i][j] as usize] += 1;
            }
        }

        for i in 1..=(n*n) {
            if seen[i] == 0 {
                res[1] = i as i32;
            }
            if seen[i] == 2 {
                res[0] = i as i32;
            }
        }
        res
    }
}