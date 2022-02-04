class Solution {
public:
    int singleNumber(vector<int>& nums) {
        // std::sort(nums.begin(), nums.end());
        // if(nums.size()<3){return nums[0];}
        // else if(nums[0]!=nums[1]){return nums[0];}
        // for(int i = 1; i<nums.size()-2; i++){
        //     if(nums[i]!=nums[i+1] && nums[i]!=nums[i-1]){return nums[i];}
        // }
        // return nums[nums.size()-1];
        
        std::unordered_set<int> num_set;
        for(int i : nums){
            auto loc = num_set.find(i);
            if(loc == num_set.end())
                num_set.insert(i);
            else
                num_set.erase(loc);
        }
        return *num_set.begin();
    }
};