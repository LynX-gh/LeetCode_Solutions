// DP O(n+m * n+m)
class Solution {
public:
    int longestCommonSubsequence(string text1, string text2) {
        int n = text1.size();
        int m = text2.size();
        vector<vector<int>> dp(n+1, vector<int>(m+1, 0));

        for (int i = 1; i <= n; i++) {
            for (int j = 1; j <= m; j++) {
                if (text1[i-1] == text2[j-1])
                    dp[i][j] = dp[i-1][j-1] + 1;
                else
                    dp[i][j] = max(dp[i-1][j], dp[i][j-1]);
            }
        }

        return dp[n][m];
    }
};

// DFS + Memo
class Solution {
public:
    int longestCommonSubsequence(string text1, string text2) {
        int n = text1.size();
        int m = text2.size();
        vector<vector<int>> dp(n, vector<int>(m, -1));
        return backtrack(text1, text2, 0, 0, dp);
    }

    int backtrack(string& text1, string& text2, int i, int j, vector<vector<int>>& dp) {
        if (i == text1.size() || j == text2.size()) {
            return 0;
        }

        if (dp[i][j] != -1) {
            return dp[i][j];
        }

        if (text1[i] == text2[j]) {
            dp[i][j] = 1 + backtrack(text1, text2, i+1, j+1, dp);
            return dp[i][j];
        }

        dp[i][j] = max(
            backtrack(text1, text2, i+1, j, dp),
            backtrack(text1, text2, i, j+1, dp)
        );
        return dp[i][j];
    }
};