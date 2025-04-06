class Solution {
public:
    bool repeatedSubstringPattern(string s) {
        string doubled = s + s;
        string sub = doubled.substr(1, doubled.size() - 2);
        return sub.find(s) != string::npos;
    }
};
