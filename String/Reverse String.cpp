class Solution {
public:
    void reverseString(vector<char>& s) {
        int beg = 0;
        int end = s.size()-1;
        while(beg <= end){
            swap(s[beg++], s[end--]);
        }
    }
};