use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut live = HashSet::new();
        let mut q = VecDeque::new();
        let mut res = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    live.insert((i, j));
                } else if grid[i][j] == 2 {
                    q.push_back((i,j,0));
                }
            }
        }

        while !live.is_empty() && !q.is_empty() {
            let (i, j, d) = q.pop_front().unwrap();
            if res != d {
                res = d;
            }
            live.remove(&(i, j));

            if i+1 < grid.len() && grid[i+1][j] == 1 {
                grid[i+1][j] = 2;
                q.push_back((i+1, j, d+1));
            }
            if i-1 < grid.len() && grid[i-1][j] == 1 {
                grid[i-1][j] = 2;
                q.push_back((i-1, j, d+1));
            }
            if j+1 < grid[0].len() && grid[i][j+1] == 1 {
                grid[i][j+1] = 2;
                q.push_back((i, j+1, d+1));
            }
            if j-1 < grid[0].len() && grid[i][j-1] == 1 {
                grid[i][j-1] = 2;
                q.push_back((i, j-1, d+1));
            }
        }

        if live.is_empty() {
            res
        } else {
            -1
        }
    }
}