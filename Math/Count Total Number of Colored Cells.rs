impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        let mut res = 1;
        let mut mul = 0;
        for i in 0..n {
            res += mul;
            mul += 4;
        }
        res
    }
}