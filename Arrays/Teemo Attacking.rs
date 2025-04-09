impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut res = 0;
        let mut last = time_series[0];

        for time in time_series {
            res += duration.min(time - last);
            last = time;
        }
        res += duration;
        res
    }
}
