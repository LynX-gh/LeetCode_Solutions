class Solution {
public:
    int removeDuplicates(vector<int>& nums) {
        int curr_idx = 0;
        int prev = 101;
        for(const int i : nums){
            if(prev != i){
                prev = i;
                nums[curr_idx] = i;
                curr_idx += 1;
            }
        }
        return curr_idx;
    }
};