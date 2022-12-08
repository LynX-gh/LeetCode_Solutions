class Solution {
public:
    int searchInsert(vector<int>& nums, int target) {
        if(nums[0] > target)
            return 0;
        int start = 0, end = nums.size()-1, mid;
        while(start <= end){
            mid = (start + end) / 2;
            if(nums[mid] == target)
                return mid;
            else if(nums[mid] < target)
                start = mid + 1;
            else
                end = mid - 1;
            }
        return start;
    }
};