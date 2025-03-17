use std::collections::BTreeSet;
use std::cmp::Reverse;

struct SmallestInfiniteSet {
    heap: BTreeSet<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {

    fn new() -> Self {
        Self { heap: BTreeSet::from_iter(1..1001) }
    }
    
    fn pop_smallest(&mut self) -> i32 {
        self.heap.pop_first().unwrap()
    }
    
    fn add_back(&mut self, num: i32) {
        self.heap.insert(num);
    }
}

/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */