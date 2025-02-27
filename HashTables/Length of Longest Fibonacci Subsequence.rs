use std::collections::HashSet;

impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let fib_map: HashSet<i32> = HashSet::from_iter(arr.iter().cloned());
        let mut res = 0;

        for m in 0..arr.len() {
            for n in m+1..arr.len() {
                let mut i = arr[m];
                let mut j = arr[n];
                let mut curr = 2;

                while(fib_map.contains(&(i as i32 + j as i32))) {
                    j = i + j;
                    i = j - i;
                    curr += 1;
                }
                res = res.max(curr);
            }
        }

        if (res > 2) { res } else { 0 }
    }
}