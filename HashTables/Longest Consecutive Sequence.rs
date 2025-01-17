// O(n)
use std::collections::HashSet;
use std::cmp;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut seen: HashSet<_> = nums.into_iter().collect();

        let mut max_seq = 0;
        for &n in &seen {
            if !seen.contains(&(n-1)) {
                let mut curr_num = n;
                let mut curr_max = 1;
                while seen.contains(&(curr_num + 1)) {
                    curr_num += 1;
                    curr_max += 1;
                }
                max_seq = cmp::max(curr_max, max_seq);
            }
        }
        max_seq
    }
}


// O(n logn)
use std::cmp;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();
        // println!("{:?}", sorted_nums);

        let mut curr_ptr = 0;
        let mut fwd_ptr = 0;
        let mut max_seq = 0;
        let mut curr_max = 0;
        
        while fwd_ptr < sorted_nums.len() {
            // println!("{} # {} $ {}", curr_ptr, fwd_ptr, curr_max);
            if sorted_nums[curr_ptr] + curr_max == sorted_nums[fwd_ptr] {
                curr_max += 1;
                fwd_ptr += 1;
            } else if sorted_nums[curr_ptr] + curr_max - 1 == sorted_nums[fwd_ptr] {
                fwd_ptr += 1;
            } else {
                max_seq = cmp::max(max_seq, curr_max);
                curr_ptr = fwd_ptr;
                curr_max = 0;
            }
        }
        cmp::max(max_seq, curr_max)
    }
}


// O(n^2)
use std::collections::HashSet;
use std::cmp;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut seen = HashSet::with_capacity(nums.len());
        let mut max_seq = 0;

        for i in 0..nums.len() {
            seen.insert(&nums[i]);
        }

        for i in 0..nums.len() {
            let mut curr_val = nums[i];
            let mut curr_max = 1;

            loop {
                if seen.contains(&(curr_val+1)) {
                    curr_max += 1;
                    curr_val += 1;
                } else {
                    break;
                }
            }

            max_seq = cmp::max(curr_max, max_seq);
        }

        max_seq
    }
}