class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int currProfit = 0;
        int maxProfit = 0;
        int lowestPoint = prices[0];
        
        if(prices.size() < 2)
            return maxProfit;
        
        for(int i = 0; i < prices.size(); i++){
            if(prices[i] < lowestPoint)
                lowestPoint = prices[i];
            
            currProfit = prices[i] - lowestPoint;
            
            if(currProfit > maxProfit)
                maxProfit = currProfit;
        }
        
        return maxProfit;
    }
};