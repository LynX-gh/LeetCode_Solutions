class Solution {
public:
    vector<int> sortedSquares(vector<int>& nums) {
        int negPtr = 0;
        int posPtr = nums.size() - 1;
        int origSize = posPtr;
        vector<int> res;
        if (posPtr == negPtr) {
            res.push_back(nums[posPtr] * nums[posPtr]);
            return res;
        }
        while (negPtr <= origSize && posPtr >= 0 && nums[negPtr] < 0 && nums[posPtr] >= 0) {
            int n1 = nums[negPtr] * nums[negPtr];
            int n2 = nums[posPtr] * nums[posPtr];
            if (n1 > n2) {
                res.push_back(n1);
                negPtr++;
            }
            else {
                res.push_back(n2);
                posPtr--;
            }
        }
        while (negPtr <= origSize && nums[negPtr] < 0) {
            res.push_back(nums[negPtr] * nums[negPtr]);
            negPtr++;
        }
        while (posPtr >= 0 && nums[posPtr] >= 0) {
            res.push_back(nums[posPtr] * nums[posPtr]);
            posPtr--;
        }
        reverse(res.begin(), res.end());
        return res;
    }
};