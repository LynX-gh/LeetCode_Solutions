class Solution {
public:
    bool checkInclusion(string s1, string s2) {
        if (s1.size() > s2.size()) 
            return false;
        vector<int> s1v (26, 0);
        for (auto c : s1) s1v[c - 'a']++;
        vector<int> s2v (26, 0);
        int l = 0, r = 0;
        while (r < s2.size()) {
            s2v[s2[r]-'a']++;
            if (r - l + 1 == s1.size()) 
                if (s1v == s2v) 
                    return true;
            if (r - l + 1 < s1.size()) r++;
            else {
                s2v[s2[l]-'a']--;
                l++;
                r++;
            }
        }
        return false;
    }
};