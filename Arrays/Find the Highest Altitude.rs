impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut alt = 0;

        for i in 0..gain.len() {
            alt += gain[i];
            res = res.max(alt);
        }
        res
    }
}