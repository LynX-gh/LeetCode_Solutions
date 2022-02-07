class Solution {
public:
    vector<string> findRestaurant(vector<string>& list1, vector<string>& list2) {
        std::ios::sync_with_stdio(false);
        vector<string> result;
        int temp = 0;
        int lowsum = INT_MAX;
        // HashMap to store list1 elems for faster lookup - string1:index1
        unordered_map<string, int> hash;
        
        for(string s : list1){
            hash[s] = temp;
            temp++;
        }
        
        for(temp = 0; temp < list2.size(); temp++){
            if(hash.find(list2[temp]) != hash.end() && hash[list2[temp]] + temp < lowsum){
                lowsum = hash[list2[temp]] + temp;
            }
        }
        
        for(temp = 0; temp < list2.size(); temp++){
            if(hash.find(list2[temp]) != hash.end() && hash[list2[temp]] + temp == lowsum){
                result.push_back(list2[temp]);
            }
        }
        
        return result;
    }
};