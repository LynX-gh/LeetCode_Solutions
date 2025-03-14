impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let mut min = 1;
        let mut max = candies.iter().fold(0, |acc, &x| acc + x as i64) / k;
        let mut res = 0;
        
        while min <= max {
            let mid = min + (max - min) / 2;
            let sum = candies.iter().fold(0, |acc, &x| acc + (x as i64 / mid));
            if sum >= k {
                res = mid;
                min = mid + 1;
            } else {
                max = mid - 1;
            }
        }
        res as i32
    }
}