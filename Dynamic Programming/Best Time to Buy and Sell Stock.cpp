class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int maxProfit = 0;
        int lowestVal = prices[0];

        for(int i = 1; i < prices.size(); i++){
            if(prices[i] < lowestVal){
                lowestVal = prices[i];
            }
            if(maxProfit < (prices[i] - lowestVal)){
                maxProfit = prices[i] - lowestVal;
            }
        }
        return maxProfit;
    }
};