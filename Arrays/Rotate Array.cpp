// InBuilt Rotate STL class
class Solution {
public:
    void rotate(vector<int>& nums, int k) {
        k = k % nums.size();
        if(nums.size() > 1){
            std::rotate(nums.rbegin(), nums.rbegin()+k, nums.rend());
        }
    }
};

// O(n), O(n)
class Solution {
public:
    void rotate(vector<int>& nums, int k) {
        k = k%nums.size();
        vector<int> res;
        int len = nums.size();
        for(int i = len-k; i < len; i++){
            res.push_back(nums[i]);
        }
        for(int i = 0; i < len-k; i++){
            res.push_back(nums[i]);
        }
        nums = res;
    }
};

// O(n), O(1)
class Solution {
public:
    void rotate(vector<int>& nums, int k) {
        int len = nums.size();
        k = k%len;

        reverse(nums.begin(), nums.end());
        reverse(nums.begin(), nums.begin()+k);
        reverse(nums.begin()+k, nums.end());
    }
};