class Solution {
public:
    int lengthOfLastWord(string s) {
        int len = s.length()-1, res = 0;
        while(len >= 0 && s[len] == ' '){
            len--;
        }
        while(len >= 0 && s[len] != ' '){
            len--;
            res++;
        }
        return res;
    }
};