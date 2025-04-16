use std::collections::HashMap;

impl Solution {
    pub fn count_good(nums: Vec<i32>, mut k: i32) -> i64 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut res = 0;

        let mut s = nums.iter();
        let mut e = nums.iter();

        while let Some(&last) = e.next() {
            let count = map.entry(last).or_insert(0);
            k -= std::mem::replace(count, *count + 1);

            while k <= 0 {
                res += e.len() as i64 + 1;
                let count = map.entry(s.next().copied().unwrap()).or_insert(0);
                *count -= 1;
                k += *count;
            }
        }

        res
    }
}
