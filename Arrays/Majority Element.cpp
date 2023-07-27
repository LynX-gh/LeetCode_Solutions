class Solution {
public:
    int majorityElement(vector<int>& nums) {
        std::unordered_map<int, int> count;
        for(const int n : nums){
            if(++count[n] > nums.size() / 2)
                return n;
        }
        return 0;
    }
};