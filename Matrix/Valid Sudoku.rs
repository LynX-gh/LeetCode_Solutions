use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = vec![0; 9];
        let mut cols = vec![0; 9];
        let mut squares = vec![0; 9];

        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] != '.' {
                    let idx = 1 << (board[i][j] as u8 - b'0');
                    if (rows[i] & idx) != 0 || (cols[j] & idx) != 0 || (squares[i/3 * 3 + j / 3] & idx) != 0 {
                        return false;
                    }
                    rows[i] |= idx;
                    cols[j] |= idx;
                    squares[i/3 * 3 + j/3] |= idx;
                }
            }
        }
        return true;
    }
}