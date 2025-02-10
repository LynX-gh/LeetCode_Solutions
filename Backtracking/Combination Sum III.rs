impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        Self::backtrack(1, n, k as usize, &mut vec![], &mut res);
        res
    }

    fn backtrack(start: i32, target: i32, max_len: usize, prev: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        for i in start..10 {
            let new_target = target - i;
            if new_target == 0 && prev.len() + 1 == max_len{
                prev.push(i);
                res.push(prev.clone());
                prev.pop();
            } else if new_target > 0 {
                prev.push(i);
                Self::backtrack(i+1, new_target, max_len, prev, res);
                prev.pop();
            }
        }
    }
}