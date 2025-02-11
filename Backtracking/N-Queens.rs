use std::collections::HashSet;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut board = vec![vec!['.'; n as usize]; n as usize];
        let mut res = vec![];
        let mut cols = HashSet::new();
        let mut pos_diag = HashSet::new();
        let mut neg_diag = HashSet::new();


        Self::backtrack(&mut res, &mut board, &mut cols, &mut pos_diag, &mut neg_diag, 0, n);
        res
    }

    fn backtrack(res: &mut Vec<Vec<String>>, 
        board: &mut Vec<Vec<char>>, 
        cols: &mut HashSet<i32>, 
        pos_diag: &mut HashSet<i32>, 
        neg_diag: &mut HashSet<i32>, 
        row: i32, 
        n: i32) 
    {
        if row == n {
            res.push(board.clone()
                .into_iter()
                .map(String::from_iter)
                .collect());
        }

        for i in 0..n {
            if !(cols.contains(&i) || pos_diag.contains(&(row-i)) || neg_diag.contains(&(row+i))) {
                cols.insert(i);
                pos_diag.insert(row-i);
                neg_diag.insert(row+i);
                board[row as usize][i as usize] = 'Q';

                Self::backtrack(res, board, cols, pos_diag, neg_diag, row+1, n);

                cols.remove(&i);
                pos_diag.remove(&(row-i));
                neg_diag.remove(&(row+i));
                board[row as usize][i as usize] = '.';
            }
        }
    }
}