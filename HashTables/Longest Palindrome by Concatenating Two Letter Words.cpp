class Solution {
public:
    int longestPalindrome(vector<string>& words) {
        unordered_map<string, int> freq;
        for (auto& word : words) {
            freq[word]++;
        }

        int ans = 0;
        bool hasCenter = false;

        for (auto& [word, count] : freq) {
            string rev = string{word[1], word[0]};
            if (word == rev) {
                ans += (count / 2) * 4;
                if (count % 2 == 1) hasCenter = true;
            } else if (freq.count(rev)) {
                int pairs = min(count, freq[rev]);
                ans += pairs * 4;
                freq[rev] = 0; // mark visited
            }
        }

        if (hasCenter) ans += 2;
        return ans;
    }
};