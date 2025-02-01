use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        if grid.len() == 0 {
            return 0;
        }

        let mut res = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    Self::bfs(i, j, &mut grid);
                    res += 1;
                }
            }
        }
        res
    }

    fn bfs(i: usize, j: usize, grid: &mut Vec<Vec<char>>) {
        let mut q = VecDeque::new();
        q.push_back((i, j));

        while( !q.is_empty() ) {
            if let Some((i, j)) = q.pop_front() {
                grid[i][j] = '0';

                let directions = [(i + 1, j), (i, j + 1), (i - 1, j), (i, j - 1)];
                for (ni, nj) in directions {
                    if ni < grid.len() && nj < grid[0].len() && grid[ni][nj] == '1' {
                        q.push_back((ni, nj));
                        grid[ni][nj] = '0';
                    }
                }
            }
        }
    }
}