class Solution {
public:
    int titleToNumber(string columnTitle) {
        int res = 0, ctr = 0;
        for(int i = columnTitle.length()-1; i >= 0; i--){
            res += (columnTitle[i] - 64) * pow(26, ctr);
            ctr++;
        }
        return res;
    }
};