class Solution {
public:
    vector<int> intersection(vector<int>& nums1, vector<int>& nums2) {
        std::unordered_set<int> nums1_set{nums1.begin(),nums1.end()};
        std::unordered_set<int> nums2_set{nums2.begin(),nums2.end()};
        
        std::vector<int> intersect;
        for (int& i: nums1_set)
            if (nums2_set.count(i)!=0)
                intersect.push_back(i);
            
        return intersect;
    }
};