use std::collections::HashMap;

impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let mut res = 2;
        let mut fib_map: HashMap<i32, usize> = arr.iter()
                                                .enumerate()
                                                .map(|(i, &x)| (x, i))
                                                .collect();
        let mut dp = vec![vec![2; arr.len()]; arr.len()];

        for j in 0..arr.len() {
            for i in 0..j {
                if let Some(&k) = fib_map.get(&(arr[j] - arr[i])) {
                    if arr[i] > arr[k] {
                        dp[i][j] = dp[k][i] + 1;
                    }
                }
                res = res.max(dp[i][j]);
            }
        }

        if (res > 2) { res } else { 0 }
    }
}