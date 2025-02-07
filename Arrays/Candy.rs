impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut res = vec![1; ratings.len()];

        for i in 1..ratings.len() {
            if ratings[i] > ratings[i-1] {
                res[i] = res[i-1] + 1;
            }
        }
        for i in (0..ratings.len()-1).rev() {
            if ratings[i] > ratings[i+1] {
                res[i] = res[i].max(res[i + 1] + 1);
            }
        }
        res.iter().sum()
    }
}