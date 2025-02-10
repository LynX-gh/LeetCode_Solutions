// Descending

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|x| x[0]);
        // points.reverse();
        // println!("{points:?}");

        let mut res = 1;
        let mut curr_arrow = points[points.len()-1][0];

        for range in points.iter().rev().skip(1) {
            if curr_arrow > range[1] {
                curr_arrow = range[0];
                res += 1;
            }
        }

        res
    }
}

// Ascending

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|x| x[0]);

        let mut res = 1;
        let mut curr_arrow = points[0][1];

        for range in points.iter().skip(1) {
            if curr_arrow < range[0] {
                curr_arrow = range[1];
                res += 1;
            } else {
                curr_arrow = curr_arrow.min(range[1]);
            }
        }

        res
    }
}