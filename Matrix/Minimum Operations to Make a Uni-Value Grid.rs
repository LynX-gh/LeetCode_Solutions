impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut arr = Vec::with_capacity(grid[0].len() * grid.len());
        let mut first = grid[0][0];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if (grid[i][j] - first) % x != 0 {
                    return -1;
                }
                arr.push(grid[i][j]);
            }
        }
        arr.sort_unstable();

        let median = arr[arr.len() / 2];
        let mut res = 0;
        for num in arr {
            res += (median - num).abs() / x;
        }
        res
    }
}