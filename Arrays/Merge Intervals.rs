use std::cmp;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut sorted_intervals = intervals.clone();
        sorted_intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut res = vec![vec![sorted_intervals[0][0], sorted_intervals[0][1]]];

        for i in 1..sorted_intervals.len() {
            let x = res[res.len()-1][0];
            let y = res[res.len()-1][1];
            let k = sorted_intervals[i][0];
            let l = sorted_intervals[i][1];

            if y >= k {
                let length = res.len();
                res[length-1] = vec![cmp::min(x,k), cmp::max(y,l)];
            } else {
                res.push(vec![k,l]);
            }
        }
        res
    }
}