class Solution {
public:
    vector<int> plusOne(vector<int>& digits) {
        int lsb = digits.size() - 1, carry = 0;
        while(lsb >= 0){
            if(digits[lsb] < 9){
                digits[lsb]++;
                break;
            }
            lsb--;
        }
        if(lsb == -1){
            digits.insert(digits.begin(), 1);
            lsb = 0;
        }
        lsb = lsb+1;
        while(lsb < digits.size() && digits[lsb] == 9){
            digits[lsb] = 0;
            lsb++;
        }
        return digits;
    }
};