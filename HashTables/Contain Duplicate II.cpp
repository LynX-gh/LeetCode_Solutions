class Solution {
public:
    bool containsNearbyDuplicate(vector<int>& nums, int k) {
        ios_base::sync_with_stdio(false);
        cin.tie(NULL);
        unordered_set<int> hash;   // num : index
        
        for(int i = 0; i < nums.size(); i++){
            if(hash.find(nums[i]) != hash.end() && i - hash[nums[i]] <= k){
                return true;
            }
            hash[nums[i]] = i;
        }
        return false;
    }
};