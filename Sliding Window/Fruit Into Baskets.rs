use std::collections::HashMap;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut basket = HashMap::new();
        let mut curr = 0;
        let mut s = 0;
        
        for i in 0..fruits.len() {
            curr += 1;
            basket.entry(fruits[i]).and_modify(|ctr| *ctr += 1).or_insert(1);

            while basket.len() > 2 {
                curr -= 1;
                if let Some(&val) = basket.get(&fruits[s]) {
                    if val == 1 {
                        basket.remove(&fruits[s]);
                    } else {
                        basket.entry(fruits[s]).and_modify(|ctr| *ctr -= 1);
                    }
                }
                s += 1;
            }
            res = res.max(curr);
        }
        res
    }
}