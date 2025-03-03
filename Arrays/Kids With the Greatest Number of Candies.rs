impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut res = vec![false; candies.len()];
        let mut max = candies.iter().max().unwrap() - extra_candies;

        for i in 0..candies.len() {
            if candies[i] >= max {
                res[i] = true;
            }
        }
        res
    }
}