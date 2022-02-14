class Solution {
public:
    vector<int> intersect(vector<int>& nums1, vector<int>& nums2) {
        ios_base::sync_with_stdio(false);
        cin.tie(NULL);
        vector<int> result;
        int temp = 0;
        unordered_map<int, int> freq;
        
        for(int& i : nums2){
            freq[i]++;
        }
        
        for(int& i : nums1){
            if(freq[i]-- > 0){
                result.push_back(i);
            }
        }
        
        return result;
    }
};