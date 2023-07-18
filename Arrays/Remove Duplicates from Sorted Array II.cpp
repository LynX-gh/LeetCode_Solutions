class Solution {
public:
    int removeDuplicates(vector<int>& nums) {
        int lastNum = 0;
        int lastNumQty = 1;
        int ret = 1;

        for(int i = 1; i < nums.size(); i++){
            if(nums[lastNum] != nums[i]){
                nums[lastNum+1] = nums[i];
                lastNum++;
                lastNumQty = 1;
                ret++;
            }
            else if(lastNumQty == 1){
                nums[lastNum+1] = nums[i];
                lastNum++;
                lastNumQty++;
                ret++;
            }
        }
        return ret;
    }
};