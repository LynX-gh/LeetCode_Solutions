class Solution {
public:
    int fourSumCount(vector<int>& nums1, vector<int>& nums2, vector<int>& nums3, vector<int>& nums4) {
        int result = 0;
        unordered_map<int, int> first_two;
        
        for(int i : nums1){
            for(int j : nums2){
                first_two[i+j]++;
            }
        }
        
        for(int i : nums3){
            for(int j : nums4){
                if(first_two.find(-i-j) != first_two.end()){ result += first_two[-i-j]; }
            }
        }
        
        return result;
    }
};