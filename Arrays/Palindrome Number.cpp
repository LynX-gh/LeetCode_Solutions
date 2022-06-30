class Solution {
public:
    bool isPalindrome(int x) {
        int temp = x;
        unsigned int revX = 0;
        while(temp > 0){
            revX = revX*10 + temp%10;
            temp = temp/10;
        }
        return revX==x;
    }
};