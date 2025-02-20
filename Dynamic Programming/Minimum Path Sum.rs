// DP O(n^2)
impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        for i in (0..grid.len()).rev() {
            for j in (0..grid[0].len()).rev() {
                let mut min_move = 0;
                if i+1 < grid.len() && j+1 < grid[0].len() {
                    min_move = grid[i+1][j].min(grid[i][j+1]);
                } else if i+1 < grid.len() {
                    min_move = grid[i+1][j];
                } else if j+1 < grid[0].len() {
                    min_move = grid[i][j+1];
                }
                grid[i][j] += min_move;
            }
        }
        grid[0][0]
    }
}


// DFS O(2^(n^2))
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut res = i32::MAX;
        Self::backtrack(&mut res, 0, 0, &mut 0, &grid);
        res
    }

    fn backtrack(res: &mut i32, i: usize, j: usize, curr: &mut i32, grid: &Vec<Vec<i32>>) {
        *curr += grid[i][j];
        if i == grid.len()-1 && j == grid[0].len()-1 {
            *res = *res.min(curr);
            *curr -= grid[i][j];
            return;
        }

        if i+1 < grid.len() {
            Self::backtrack(res, i+1, j, curr, grid);
        }

        if j+1 < grid[0].len() {
            Self::backtrack(res, i, j+1, curr, grid);
        }
        *curr -= grid[i][j];
    }
}