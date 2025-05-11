use std::collections::BinaryHeap;
use std::cmp::{Reverse, max};

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let n = move_time.len();
        let m = move_time[0].len();
        let inf = i32::MAX / 2;
        let mut d = vec![vec![inf; m]; n];
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        d[0][0] = 0;
        let mut q = BinaryHeap::new();
        q.push(Reverse((0, 0usize, 0usize)));
        while let Some(Reverse((dis, x, y))) = q.pop() {
            if dis != d[x][y] {
                continue;
            }
            if x == n - 1 && y == m - 1 {
                break;
            }
            for &(dx, dy) in dirs.iter() {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx < 0 || nx >= n as isize || ny < 0 || ny >= m as isize {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                let nd = max(d[x][y], move_time[nx][ny]) + (((x + y) % 2) as i32 + 1);
                if d[nx][ny] > nd {
                    d[nx][ny] = nd;
                    q.push(Reverse((nd, nx, ny)));
                }
            }
        }
        d[n - 1][m - 1]
    }
}