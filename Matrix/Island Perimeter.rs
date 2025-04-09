impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut res = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    if i + 1 >= grid.len() || grid[i+1][j] == 0 {
                        res += 1;
                    }
                    if i-1 >= grid.len() || grid[i-1][j] == 0 {
                        res += 1;
                    }
                    if j+1 >= grid[0].len() || grid[i][j+1] == 0 {
                        res += 1;
                    }
                    if j-1 >= grid[0].len() || grid[i][j-1] == 0 {
                        res += 1;
                    }
                }
            }
        }
        res
    }
}
