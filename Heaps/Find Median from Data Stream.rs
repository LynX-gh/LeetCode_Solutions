use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MedianFinder {
    small_heap: BinaryHeap<i32>,
    large_heap: BinaryHeap<Reverse<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    fn new() -> Self {
        Self {
            small_heap: BinaryHeap::new(),
            large_heap: BinaryHeap::new(),
        }
    }
    
    fn add_num(&mut self, num: i32) {
        self.small_heap.push(num);
        self.large_heap.push(Reverse(*self.small_heap.peek().unwrap()));
        self.small_heap.pop();
        if (self.small_heap.len() < self.large_heap.len()) {
            self.small_heap.push(self.large_heap.peek().unwrap().0);
            self.large_heap.pop();
        }
    }
    
    fn find_median(&self) -> f64 {
        if self.small_heap.len() > self.large_heap.len() {
            return *self.small_heap.peek().unwrap() as f64;
        } 
        (self.small_heap.peek().unwrap() + self.large_heap.peek().unwrap().0) as f64 / 2.0
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */