class Solution {
public:
    string minWindow(string s, string t) {
        vector<int> cmap(128, 0);
        for (const char c : t) cmap[c]++;
        int counter = t.size(), start = 0, end = 0, res = INT_MAX, res_start = 0;

        while (end < s.length()) {
            if (cmap[s[end++]]-- > 0) {
                counter--;
            }

            while (counter == 0) {
                if (end - start < res) {
                    res = end-start;
                    res_start = start;
                }

                if (cmap[s[start++]]++ == 0) {
                    counter++;
                }
            }
        }

        return res == INT_MAX ? "" : s.substr(res_start, res);
    }
};