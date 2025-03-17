use std::collections::{VecDeque, HashSet};

impl Solution {
    pub fn nearest_exit(mut maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let ei = entrance[0] as usize;
        let ej = entrance[1] as usize;
        maze[ei][ej] = '+';

        let mut q = VecDeque::new();
        q.push_back((ei+1, ej, 1));
        q.push_back((ei-1, ej, 1));
        q.push_back((ei, ej+1, 1));
        q.push_back((ei, ej-1, 1));

        while let Some((i,j,d)) = q.pop_front() {
            if i < maze.len() && j < maze[0].len() && maze[i][j] != '+' {
                maze[i][j] = '+';
                if (i == 0 || i == maze.len()-1 || j == 0 || j == maze[0].len()-1) {
                    return d;
                }
                q.push_back((i+1, j, d+1));
                q.push_back((i-1, j, d+1));
                q.push_back((i, j+1, d+1));
                q.push_back((i, j-1, d+1));
            }
        }
        -1
    }
}