class Solution {
public:
    pair<int, int> directions[4] = {{1, 0}, {0, 1}, {-1, 0}, {0, -1}};

    int numIslands(vector<vector<char>>& grid) {
        if (grid.size() == 0) {
            return 0;
        }

        int res = 0;

        for(int i = 0; i < grid.size(); i++) {
            for(int j = 0; j < grid[0].size(); j++) {
                if (grid[i][j] == '1') {
                    bfs(i, j, grid);
                    res++;
                }
            }
        }
        return res;
    }

    void bfs(int i, int j, vector<vector<char>>& grid) {
        queue<pair<int, int>> q;
        q.push({i, j});

        while(!q.empty()) {
            int i = q.front().first;
            int j = q.front().second;
            q.pop();
            grid[i][j] = '0';

            for(int k = 0; k < 4; k++) {
                int ni = i + directions[k].first;
                int nj = j + directions[k].second;

                if (ni < grid.size() && ni >= 0 && nj < grid[0].size() && nj >= 0 && grid[ni][nj] == '1') {
                    q.push({ni, nj});
                    grid[ni][nj] = '0';
                }
            }
        }
    }
};