// Optimized DP
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut b1 = i32::MAX;
        let mut s1 = 0;
        let mut b2 = i32::MAX;
        let mut s2 = 0;

        for num in prices {
            b1 = b1.min(num);
            s1 = s1.max(num - b1);
            b2 = b2.min(num - s1);
            s2 = s2.max(num - b2);
        }

        s2
    }
}

// DFS + Memo
use std::collections::HashMap;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        Self::backtrack(0, 0, &prices, &mut map)
    }

    fn backtrack(idx: usize, state: usize, prices: &Vec<i32>, map: &mut HashMap<(usize, usize), i32>) -> i32 {
        if idx == prices.len() {
            return 0;
        }

        if let Some(&val) = map.get(&(idx, state)) {
            return val;
        }

        let do_nothing = Self::backtrack(idx + 1, state, prices, map);
        let mut max_profit = do_nothing;

        match state {
            0 | 2 => { // Buy first or second stock
                let buy = Self::backtrack(idx + 1, state + 1, prices, map) - prices[idx];
                max_profit = max_profit.max(buy);
            }
            1 | 3 => { // Sell first or second stock
                let sell = Self::backtrack(idx + 1, state + 1, prices, map) + prices[idx];
                max_profit = max_profit.max(sell);
            }
            _ => {}
        }

        map.insert((idx, state), max_profit);
        max_profit
    }
}