impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut start = prices[0];
        let mut res = 0;

        for i in 1..prices.len() {
            if prices[i-1] < prices[i] {
                res += prices[i] - prices[i-1];
            }
        }
        res
    }
}