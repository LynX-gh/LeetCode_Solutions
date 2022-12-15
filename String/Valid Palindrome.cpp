class Solution {
public:
    bool isPalindrome(string s) {
        transform(s.begin(), s.end(), s.begin(),[](unsigned char c){ return std::tolower(c); });
        if(s.length() == 1){ return true; }
        string clean = "";
        for(char ch : s){
            if(isalnum(ch)){
                clean += ch;
            }
        }
        s = clean;
        reverse(s.begin(), s.end());
        return clean == s;
    }
};