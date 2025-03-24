impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        intervals.sort_by_key(|interval| interval[1]);

        let mut end = i32::MIN;
        for i in 0..intervals.len() {
            if intervals[i][0] >= end {
                end = intervals[i][1]
            } else {
                res += 1;
            }
        }
        res
    }
}