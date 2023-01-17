class Solution {
public:
    void rotate(vector<int>& nums, int k) {
        k = k % nums.size();
        if(nums.size() > 1){
            std::rotate(nums.rbegin(), nums.rbegin()+k, nums.rend());
        }
    }
};