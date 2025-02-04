use std::collections::VecDeque;

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let mut visited = vec![vec![false; n]; n];
        let mut deque: VecDeque<(usize, i32)> = VecDeque::new();

        deque.push_back((1, 0));
        visited[n - 1][0] = true;

        while !deque.is_empty() {
            let (curr, steps) = deque.pop_front().unwrap();

            if curr == n * n {
                return steps;
            }

            for next in curr + 1..std::cmp::min(curr + 6, n * n) + 1 {
                let (x, y) = Self::get_coordinate(next, n);

                if !visited[x][y] {
                    let pos = if board[x][y] == -1 { next } else { board[x][y] as usize };
                    deque.push_back((pos, steps + 1));
                    visited[x][y] = true;
                }
            }
        }

        -1
    }

    fn get_coordinate(pos: usize, n: usize) -> (usize, usize) {
        let quote = (pos - 1) / n;
        let remainder = (pos - 1) % n;

        let row = n - 1 - quote;
        let col = if row % 2 != n % 2 { remainder } else { n - 1 - remainder };

        (row, col)
    }
}