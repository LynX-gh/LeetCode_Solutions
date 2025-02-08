class Solution {
private: 
    bool solve(int i, int j, int ind, int m, int n, string& word, vector<vector<char>>& board, vector<vector<int>>& vis) {
        if(ind == word.size()) return true;

        if(i < 0 || j < 0 || i >= m || j >= n || vis[i][j] == true || board[i][j] != word[ind]) return false;

        vis[i][j] = true;

        if(solve(i-1, j, ind+1, m, n, word, board, vis)) return true;
        if(solve(i+1, j, ind+1, m, n, word, board, vis)) return true;
        if(solve(i, j+1, ind+1, m, n, word, board, vis)) return true;
        if(solve(i, j-1, ind+1, m, n, word, board, vis)) return true;

        vis[i][j] = false;
        return false;
    }
    
public:
    bool exist(vector<vector<char>>& board, string word) {
        int m = board.size();
        int n = board[0].size();

        for(int i = 0; i < m; i++) {
            for(int j = 0; j < n; j++) {
                if(board[i][j] == word[0]) {
                    vector<vector<int>> vis (m, vector<int> (n, 0));
                    if(solve(i, j, 0, m, n, word, board, vis)) return true;
                }
            }
        }
        return false;
    }
};