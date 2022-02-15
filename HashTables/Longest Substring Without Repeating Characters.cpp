class Solution {
public:
    int lengthOfLongestSubstring(string s) {
        // Map to store seen values and its index
        unordered_map<char, int> seen;
        // Sliding Window Algo
        int left = 0;
        int right = 0;

        int result = 0;
        
        while (right < s.length()) {
            if (seen.find(s[right]) != seen.end() && seen[s[right]] >= left && seen[s[right]] < right){
                left = seen[s[right]] + 1;
            }
            result = max(result, right - left + 1);

            seen[s[right]] = right;
            right++;
        }
        return result;
		
	/*
		// we will store a senitel value of -1 to simulate 'null'/'None' in C++
        vector<int> chars(128, -1);

        int left = 0;
        int right = 0;

        int res = 0;
        while (right < s.length()) {
            char r = s[right];

            int index = chars[r];
            if (index != -1 and index >= left and index < right) {
                left = index + 1;
            }
            res = max(res, right - left + 1);

            chars[r] = right;
            right++;
        }
        return res;
	*/
    }
};