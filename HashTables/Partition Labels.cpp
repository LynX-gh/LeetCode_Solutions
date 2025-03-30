class Solution {
public:
    vector<int> partitionLabels(string s) {
        vector<int> orig_map(26, 0);
        vector<int> part_map(26, 0);

        for (const auto ch: s) {
            orig_map[ch - 'a']++;
            part_map[ch - 'a']++;
        }

        vector<int> res;
        int i = 0;
        for (int j = 0; j < s.length(); j++) {
            part_map[s[j] - 'a']--;
            if (part_map[s[j] - 'a'] == 0) {
                bool part = true;
                for (int k = 0; k < 26; k++) {
                    if (part_map[k] != 0 && part_map[k] != orig_map[k]) {
                        part = false;
                        break;
                    }
                }
                if (part) {
                    res.push_back(j - i + 1);
                    i = j+1;
                }
            }
        }
        return res;
    }
};
