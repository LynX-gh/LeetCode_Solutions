use std::collections::BinaryHeap;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut order_set = BinaryHeap::new();
        let mut res = 0;

        for val in nums {
            order_set.push(val);
        }

        for _ in 0..k {
            res = order_set.pop().unwrap();
        }
        res
    }
}