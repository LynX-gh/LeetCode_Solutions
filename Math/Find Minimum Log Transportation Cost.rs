impl Solution {
    pub fn min_cutting_cost(n i32, m i32, k i32) - i64 {
        let mut res = 0;
        if n  k {
            res += (n-k) as i64  k as i64; 
        }
        if m  k {
            res += (m-k) as i64  k as i64;
        }
        res
    }
}