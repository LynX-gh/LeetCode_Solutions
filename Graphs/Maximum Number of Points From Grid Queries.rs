use std::collections::{HashSet, BinaryHeap};

impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut query_idx: Vec<(usize, i32)> = queries.iter().enumerate().map(|(i, &q)| (i, q)).collect();
        query_idx.sort_unstable_by_key(|&(_, q)| q);
        let mut res = vec![0; queries.len()];
        
        Self::bfs(&grid, &query_idx, &mut res);
        res
    }

    fn bfs(grid: &Vec<Vec<i32>>, queries: &Vec<(usize, i32)>, res: &mut Vec<i32>) {
        let mut heap = BinaryHeap::new();
        let mut seen = HashSet::new();
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut count = 0;
        
        heap.push((-(grid[0][0]), 0, 0)); // Min-heap using negative values
        seen.insert((0, 0));
        
        let mut idx = 0;
        while idx < queries.len() {
            let query_value = queries[idx].1;
            
            while let Some((val, x, y)) = heap.pop() {
                let val = -val; // Convert back to positive
                if val >= query_value {
                    heap.push((-val, x, y)); // Put back the element
                    break;
                }
                
                count += 1;
                for &(dx, dy) in &directions {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx >= 0 && ny >= 0 && (nx as usize) < grid.len() && (ny as usize) < grid[0].len() {
                        let next_pos = (nx as usize, ny as usize);
                        if seen.insert(next_pos) {
                            heap.push((-(grid[nx as usize][ny as usize]), nx as usize, ny as usize));
                        }
                    }
                }
            }
            res[queries[idx].0] = count;
            idx += 1;
        }
    }
}
