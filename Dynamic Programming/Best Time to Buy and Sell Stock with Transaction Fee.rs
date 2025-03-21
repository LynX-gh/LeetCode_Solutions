impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut buy = vec![0; prices.len()+1];
        let mut sell = vec![0; prices.len()+1];

        buy[0] = -prices[0];
        for i in 1..prices.len() {
            buy[i] = buy[i-1].max(sell[i-1]-prices[i]);
            sell[i] = sell[i-1].max(buy[i-1]+prices[i]-fee);
        }
        sell[prices.len()-1]
    }
}