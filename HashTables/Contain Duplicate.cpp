class Solution {
public:
    bool containsDuplicate(vector<int>& nums) {
        std::unordered_set<int> int_set;
        for(int num : nums){
            if(int_set.find(num) == int_set.end())
                int_set.insert(num);
            else
                return true;
        }
        return false;
    }
};