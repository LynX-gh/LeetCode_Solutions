use std::collections::BinaryHeap;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        for num in nums {
            heap.push(num);
        }
        for i in 0..k-1 {
            heap.pop();
        }
        *heap.peek().unwrap()
    }
}
