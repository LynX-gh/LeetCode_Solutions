use std::collections::HashMap;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut map = HashMap::new();
        for i in 1..=n {
            let mut j = i;
            let mut sum = 0;
            while j >= 1 {
                sum += j%10;
                j /= 10;
            }
            map.entry(sum).and_modify(|ctr| *ctr += 1).or_insert(1);
        }

        let mut max = 0;
        let mut res = 0;
        for &val in map.values() {
            if val > max {
                res = 1;
                max = val;
            } else if val == max {
                res += 1;
            }
        }
        res
    }
}
