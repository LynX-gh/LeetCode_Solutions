impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        
        let mut total: i64 = 0;
        let mut row_sums = vec![0i64; m];
        let mut col_sums = vec![0i64; n];

        for i in 0..m {
            for j in 0..n {
                let v = grid[i][j] as i64;
                total += v;
                row_sums[i] += v;
                col_sums[j] += v;
            }
        }

        if total % 2 != 0 {
            return false;
        }
        let half = total / 2;

        let mut acc = 0i64;
        for i in 0..(m - 1) {
            acc += row_sums[i];
            if acc == half {
                return true;
            }
        }

        acc = 0;
        for j in 0..(n - 1) {
            acc += col_sums[j];
            if acc == half {
                return true;
            }
        }

        false
    }
}