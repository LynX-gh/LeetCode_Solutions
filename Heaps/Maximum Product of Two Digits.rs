use std::collections::BinaryHeap;

impl Solution {
    pub fn max_product(mut n: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        while n > 0 {
            heap.push(n%10);
            n /= 10;
        }

        heap.pop().unwrap() * heap.pop().unwrap()
    }
}