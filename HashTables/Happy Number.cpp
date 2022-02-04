class Solution {
private:
    int getNext(int n){
        int sum = 0;
        while(n>0){
            int d = n%10;
            n /= 10;
            sum += d*d;
        }
        return sum;
    }
    
public:
    bool isHappy(int n) {
        std::unordered_set<int> seen;
        while(n != 1 && seen.count(n) == 0){
            seen.insert(n);
            n = getNext(n);
        }
        return n == 1;
    }
};