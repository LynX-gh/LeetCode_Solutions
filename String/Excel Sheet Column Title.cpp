class Solution {
public:
    string convertToTitle(int columnNumber) {
        string res = "";
        while(columnNumber > 0){
            columnNumber--;
            res += char(columnNumber%26 + 65);
            columnNumber /= 26;
        }
        reverse(res.begin(), res.end());
        return res;
    }
};