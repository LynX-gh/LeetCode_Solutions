class Solution {
public:
    string longestCommonPrefix(vector<string>& strs) {
        if (strs.empty()) 
            return "";
        
        string res = "";
        
        for(int i = 0; i < strs[0].length(); i++){
            for(int j = 1; j < strs.size(); j++){
                if(strs[j].length() == i || strs[j][i] != strs[0][i])
                    return res;
            }
            res += strs[0][i];
        }
        return res;
    }
};

// Coveted 0ms solution with 200 iq

class Solution {
public:
    string longestCommonPrefix(vector<string>& strs) {
        string ans="";
        sort(strs.begin(),strs.end());
        int n=strs.size();
        for(int i=0;i<min(strs[0].size(),strs[n-1].size());i++){
            if(strs[0][i]!=strs[n-1][i]){
                return ans;
            }
            ans+=strs[0][i];
        }
        return ans;
    }
};