class Solution {
public:
    int addDigits(int num) {
        if(num < 10){
            return num;
        }
        else{
            int res = 0;
            while(num > 0){
                res += num % 10;
                num /= 10;
            }
            return addDigits(res);
        }
    }
};