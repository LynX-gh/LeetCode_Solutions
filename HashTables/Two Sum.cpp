class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        vector<int> result;
        // Hash map to store index of numbers already encountered
        std::unordered_map<int, int> hash; // num : index
        
        for(int i = 0; i < nums.size(); i++){
            if(hash.count(target-nums[i]) != 0){
                result.push_back(i);
                result.push_back(hash[target-nums[i]]);
                break;
            }
            hash[nums[i]] = i;
        }
        return result;
    }
};