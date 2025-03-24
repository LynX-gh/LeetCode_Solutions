impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stk: Vec<(i32, usize)> = Vec::new();
        let mut res = vec![0; temperatures.len()];

        for i in 0..temperatures.len() {
            while !stk.is_empty() && stk[stk.len()-1].0 < temperatures[i] {
                let (_, idx) = stk.pop().unwrap();
                res[idx] = (i - idx) as i32;
            }
            stk.push((temperatures[i], i));
        }
        res
    }
}