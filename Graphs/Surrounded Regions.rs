impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if (i == 0 || j == 0 || i == board.len()-1 || j == board[0].len()-1) && board[i][j] == 'O' {
                    Self::dfs_edge(i, j, board);
                }
            }
        }

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == 'O' {
                    board[i][j] = 'X';
                } else if board[i][j] == 'T' {
                    board[i][j] = 'O';
                }
            }
        }
    }

    fn dfs_edge(i: usize, j: usize, board: &mut Vec<Vec<char>>) {
        if i < board.len() && j < board[0].len() && board[i][j] == 'O' {
            board[i][j] = 'T'; 
            Self::dfs_edge(i + 1, j, board);
            Self::dfs_edge(i, j + 1, board);
            Self::dfs_edge(i - 1, j, board);
            Self::dfs_edge(i, j - 1, board);
        }
    }
}