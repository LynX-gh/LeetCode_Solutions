class Solution {
public:
    vector<vector<string>> groupAnagrams(vector<string>& strs) {
        ios_base::sync_with_stdio(false);
        cin.tie(NULL);
        vector<vector<string>> result;
        unordered_map<string, vector<string>> hash;
        
        for(string s : strs){
            string sorted_s = s;
            sort(sorted_s.begin(), sorted_s.end());
            hash[sorted_s].push_back(s);
        }
        
        for(auto& elem : hash){
            result.push_back(elem.second);
        }
        
        return result;
    }
};