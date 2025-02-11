use std::collections::HashSet;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut cols = HashSet::new();
        let mut pos_diag = HashSet::new();
        let mut neg_diag = HashSet::new();

        Self::backtrack(&mut cols, &mut pos_diag, &mut neg_diag, 0, n)
    }

    fn backtrack(cols: &mut HashSet<i32>,
                    pos_diag: &mut HashSet<i32>,
                    neg_diag: &mut HashSet<i32>,
                    row: i32,
                    n: i32) -> i32
    {
        if row == n {
            return 1;
        }

        let mut res = 0;
        for i in 0..n {
            if !(cols.contains(&i) || pos_diag.contains(&(row-i)) || neg_diag.contains(&(row+i))) {
                cols.insert(i);
                pos_diag.insert(row-i);
                neg_diag.insert(row+i);

                res += Self::backtrack(cols, pos_diag, neg_diag, row+1, n);

                cols.remove(&i);
                pos_diag.remove(&(row-i));
                neg_diag.remove(&(row+i));
            }
        }
        res
    }
}