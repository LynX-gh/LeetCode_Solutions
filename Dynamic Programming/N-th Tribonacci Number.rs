class Solution {
private:
    vector<int> dp;
public:
    Solution() : dp(38, -1) {};
    int tribonacci(int n) {
        if (n == 0) {
            return 0;
        }
        if (n == 1) {
            return 1;
        }
        if (n == 2) {
            return 1;
        }
        if (dp[n] != -1) {
            return dp[n];
        }
        
        dp[n] = tribonacci(n-3) + tribonacci(n-2) + tribonacci(n-1);
        return dp[n];
    }
};