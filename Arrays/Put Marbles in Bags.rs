impl Solution {
    pub fn put_marbles(weights: Vec<i32>, k32: i32) -> i64 {
        let k = k32 as usize;

        let mut splits = Vec::with_capacity(weights.len()-1);

        for i in 0..weights.len()-1 {
            splits.push((weights[i] + weights[i+1]) as i64);
        }
        splits.sort_unstable();

        let mut res = 0;
        for i in 0..k-1 {
            res -= splits[i];
            res += splits[splits.len()-1-i];
        }
        res
    }
}
