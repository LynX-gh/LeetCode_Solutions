use std::cmp;

impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();

        for interval in intervals.into_iter() {
            if new_interval[1] < interval[0] {
                res.push(new_interval);
                new_interval = interval;
            }
            else if new_interval[0] > interval[1] {
                res.push(interval);
            }
            else {
                new_interval[0] = cmp::min(new_interval[0], interval[0]);
                new_interval[1] = cmp::max(new_interval[1], interval[1]);
            }
        }
        res.push(new_interval);
        res
    }
}