impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut res = Vec::new();
        for i in 0..=n as usize {
            res.push(i.count_ones() as i32);
        }
        res
    }
}