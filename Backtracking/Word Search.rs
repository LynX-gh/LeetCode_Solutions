use stdcollectionsHashSet;

impl Solution {
    pub fn exist(board VecVecchar, mut word String) - bool {
        word = word.chars().rev().collectString();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                let mut visited = HashSetnew();
                if board[i][j] == word.chars().next().unwrap() && Selfdfs(i, j, 0, &board, &word, &mut visited) {
                    return true;
                }
            }
        }
        false
    }

    fn dfs(i usize, j usize, curr usize, board &VecVecchar, word &String, visited &mut HashSet(usize, usize)) - bool {
        if curr == word.len() {
            return true;
        }
        
        if i  board.len() && j  board[0].len() && !visited.contains(&(i, j)) && curr  word.len() && board[i][j] == word.chars().nth(curr).unwrap() {
            visited.insert((i, j));

            if Selfdfs(i+1, j, curr+1, board, word, visited) 
                Selfdfs(i-1, j, curr+1, board, word, visited) 
                Selfdfs(i, j+1, curr+1, board, word, visited) 
                Selfdfs(i, j-1, curr+1, board, word, visited) {
                    return true;
            }

            visited.remove(&(i, j));
        }

        false
    }
}
