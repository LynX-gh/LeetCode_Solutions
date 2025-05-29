class Solution {
public:
    string resultingString(string s) {
        string res;
        res.reserve(s.size());
        for (char c : s) {
            if (!res.empty()) {
                char a = res.back();
                if (abs(a - c) == 1 || (a == 'a' && c == 'z') || (a == 'z' && c == 'a')) {
                    res.pop_back();
                    continue;
                }
            }
            res.push_back(c);
        }
        return res;
    }
};