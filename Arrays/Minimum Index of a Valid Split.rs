// Without HashMap

impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let mut cnt = 0;
        let mut dom_num = 0;
        for &num in &nums {
            if cnt == 0 {
                dom_num = num;
            }
            if dom_num == num {
                cnt += 1;
            } else {
                cnt -= 1;
            }
        }

        let n = nums.len();
        let tot = nums.iter().filter(|&&x| x == dom_num).count();

        if n == tot * 2 - 1 {
            return -1;
        }

        let mut cnt = 0;
        for (i, &num) in nums.iter().enumerate() {
            if num == dom_num {
                cnt += 1;
            }

            if cnt * 2 > i + 1 && (tot - cnt) * 2 > n - i - 1 {
                return i as i32;
            }
        }

        -1
    }
}

// With HashMap

use std::collections::HashMap;

impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let mut cnt = HashMap::new();
        for &num in &nums {
            cnt.entry(num).and_modify(|i| *i += 1).or_insert(1);
        }

        let mut d = 0;
        let mut d_cnt = 0;
        for (&key, &val) in cnt.iter() {
            if val > d_cnt {
                d_cnt = val;
                d = key;
            }
        }

        let mut cum_cnt = 0;
        for i in 0..nums.len() {
            if nums[i] == d {
                cum_cnt += 1;
            }
            if (cum_cnt * 2) > i + 1 && (d_cnt - cum_cnt) * 2 > nums.len() - i - 1 {
                return i as i32;
            }
        }
        -1
    }
}
