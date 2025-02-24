impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = prices[0];
        let mut res = 0;

        for num in prices {
            min = min.min(num);
            res = res.max(num - min);
        }

        res
    }
}