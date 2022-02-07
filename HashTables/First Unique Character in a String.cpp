/* class Solution {
public:
    int firstUniqChar(string s) {
        int flag = 1;
        char c;
        for(int i = 0; i < s.length(); i++){
            c = s[i];
            flag = 1;
            for(int j = 0; j < s.length(); j++){
                if(i == j){continue;}
                else if(c == s[j]){
                    flag = 0;
                    break;
                }
            }
            if(flag > 0){return i;}
        }
        return -1;
    }
}; */

class Solution {
public:
    int firstUniqChar(string s) {
        ios_base::sync_with_stdio(false);
        unordered_map<char, int> hash;
        
        for(int i = 0; i < s.length(); i++){
            hash[s[i]]++;
        }
        
        for(int i = 0; i < s.length(); i++){
            if(hash[s[i]] == 1)
                return i;
        }
        
        return -1;
    }
};