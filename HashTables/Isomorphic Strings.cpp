class Solution {
public:
    bool isIsomorphic(string s, string t) {
        // Maps to store encountered pairs
        unordered_map<char, char> hash_s_t, hash_t_s; // x[i] : y[i]
        
        for(int i = 0; i < s.length(); i++){
            auto check_s = hash_s_t.find(s[i]);
            auto check_t = hash_t_s.find(t[i]);
            
            if(check_s != hash_s_t.end()){
                if(check_s->second != t[i])
                    return false;
            }
            else{hash_s_t[s[i]] = t[i];}
            
            if(check_t != hash_t_s.end()){
                if(check_t->second != s[i])
                    return false;
            }
            else{hash_t_s[t[i]] = s[i];}
        }
        return true;
    }
};