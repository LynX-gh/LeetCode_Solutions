class Solution {
public:
    int removeElement(vector<int>& nums, int val) {
        int len = nums.size()-1;
        int removals = 0;
        int i = 0, j = 0;
        while(j <= len){
            if(nums[i] == val){
                nums[i] = nums[len-removals];
                removals++;
                j++;
            }
            else{
                j++;
                i++;
            }
        }
        return i;
    }
};