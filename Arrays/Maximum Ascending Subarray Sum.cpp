class Solution {
public:
    int maxAscendingSum(vector<int>& nums) {
        int res = 0;
        int cur_sum = nums[0];

        for (int i = 1; i < nums.size(); i++) {
            if (nums[i] > nums[i-1]) {
                cur_sum += nums[i];
            } else {
                res = max(res, cur_sum);
                cur_sum = nums[i];
            }
        }
        return max(res, cur_sum);
    }
};