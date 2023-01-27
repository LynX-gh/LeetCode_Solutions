class Solution {
public:
vector<vector<int>> floodFill(vector<vector<int>> &image, int sr, int sc, int color) {
    int n = image.size();
    int m = image[0].size();
    int initial = image[sr][sc];
    if(color == initial) return image;
    
    queue<pair<int, int>> q;
    q.push({sr, sc});
    image[sr][sc] = color;

    while (!q.empty()) {
        int row = q.front().first;
        int col = q.front().second;
        q.pop();

        for (int i = -1; i <= 1; i++) {
            for (int j = -1; j <= 1; j++) {
                if (abs(i - j) != 1) continue;

                int dr = row + i;
                int dcol = col + j;
                if (dr < 0 || dr >= n || dcol < 0 || dcol >= m) continue;

                if (image[dr][dcol] == initial) {
                    q.push({dr, dcol});
                    image[dr][dcol] = color;
                }
            }
        }
    }
    return image;
    }
};