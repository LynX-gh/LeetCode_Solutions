use std::collections::BinaryHeap;

impl Solution {
    pub fn max_sum(mut grid: Vec<Vec<i32>>, mut limits: Vec<i32>, mut k: i32) -> i64 {
        let mut heap = BinaryHeap::new();
        let mut res = 0;
        
        for i in 0..grid.len() {
            grid[i].sort();
            let mut j = grid[i].len()-1;
            while limits[i] > 0 && j < grid[i].len() {
                heap.push(grid[i][j]);
                limits[i] -= 1;
                j -= 1;
            }
        }
        // println!("{:?}", heap);

        for i in 0..k {
            if let Some(val) = heap.pop() {
                res += val as i64;
            }
        }
        res
    }
}