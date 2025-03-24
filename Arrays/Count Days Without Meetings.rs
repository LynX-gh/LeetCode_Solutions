use std::collections::HashSet;

impl Solution {
    pub fn count_days(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        let mut res = days;
        meetings.sort_by_key(|meet| meet[0]);

        let mut end = 0;
        let mut i = 1;
        while i < meetings.len() {
            if meetings[i][0] <= meetings[end][1] {
                meetings[end][0] = meetings[i][0].min(meetings[end][0]);
                meetings[end][1] = meetings[i][1].max(meetings[end][1]);
            } else {
                res -= meetings[end][1] - meetings[end][0] + 1;
                end = i;
            }
            i += 1;
        }
        res -= meetings[end][1] - meetings[end][0] + 1;
        res
    }
}