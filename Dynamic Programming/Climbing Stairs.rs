// DP

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 0 { return 0; }
        if n == 1 { return 1; } 
        if n == 2 { return 2; }

        let mut minus_one_step = 2;
        let mut minus_two_step = 1;
        let mut curr = 0;

        for _ in 2..n {
            curr = minus_one_step + minus_two_step;
            minus_two_step = minus_one_step;
            minus_one_step = curr;
        }
        curr
    }
}

// Memoization Bottom Up
use std::collections::HashMap;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut map = HashMap::with_capacity(n as usize);
        Self::backtrack(&mut map, n)
    }

    fn backtrack(map: &mut HashMap<i32, i32>, target: i32) -> i32 {
        if target == 0 {
            1
        } else if target < 0 {
            0
        } else if let Some(&val) = map.get(&target) {
            val
        } else {
            let res = Self::backtrack(map, target-1) + Self::backtrack(map, target-2);
            map.insert(target, res);
            res
        }
    }
}