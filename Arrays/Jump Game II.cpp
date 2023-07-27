class Solution {
public:
    int jump(vector<int>& nums) {
        int r = 0, res = 0, farthest = 0;
        for(int i = 0; i < nums.size()-1; i++){
            farthest = max(farthest, i+nums[i]);
            if(i==r){
                r = farthest;
                res++;
            }
        }
        return res;
    }
};