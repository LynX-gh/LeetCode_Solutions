class Solution {
public:
    bool isSubsequence(string s, string t) {
        if(s.empty()){
            return true;
        }
        if(t.empty()){
            return false;
        }
        int sPtr = 0;
        int tPtr = 0;
        while(sPtr < s.length()){
            if(tPtr > t.length()-1){
                return false;
            }
            if(t[tPtr] == s[sPtr]){
                sPtr++;
            }
            tPtr++;
        }
        return true;
    }
};