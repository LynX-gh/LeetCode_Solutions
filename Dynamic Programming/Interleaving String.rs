impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }

        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let s3: Vec<char> = s3.chars().collect();

        let mut dp = vec![vec![false; s2.len()+1]; s1.len()+1];
        dp[s1.len()][s2.len()] = true;

        for i in (0..=s1.len()).rev() {
            for j in (0..=s2.len()).rev() {
                if i < s1.len() && s3[i + j] == s1[i] && dp[i + 1][j] {
                    dp[i][j] = true;
                }

                if j < s2.len() && s3[i + j] == s2[j] && dp[i][j + 1] {
                    dp[i][j] = true;
                }
            }
        }

        dp[0][0]
    }
}