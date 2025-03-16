impl Solution {
    pub fn repair_cars(mut ranks: Vec<i32>, cars: i32) -> i64 {
        // ranks.sort();
        let n = cars as i64;

        let mut min = *ranks.iter().min().unwrap() as i64;
        let mut max = *ranks.iter().min().unwrap() as i64 * n * n;
        let mut res = max;

        while min <= max {
            let mid = min + (max - min) / 2;

            let mut cur = 0;
            let mut mech = 0;
            while cur < n && mech < ranks.len() {
                cur += (mid / ranks[mech] as i64).isqrt();
                mech += 1;
            }

            if cur >= n {
                res = mid;
                max = mid - 1;
            } else {
                min = mid + 1;
            }
        }
        res
    }
}