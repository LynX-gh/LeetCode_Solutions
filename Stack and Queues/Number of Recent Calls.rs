use std::collections::VecDeque;

// Two Pointer O(1) | O(n)
struct RecentCounter {
    q: Vec<i32>,
    s: usize,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        Self {
            q: Vec::new(),
            s: 0,
        }
    }
    
    fn ping(&mut self, t: i32) -> i32 {
        while self.s < self.q.len() && self.q[self.s] < t - 3000 {
            self.s += 1;
        }
        self.q.push(t);
        (self.q.len() - self.s) as i32
    }
}


// Queue O(1) | O(k)
struct RecentCounter {
    q: VecDeque<i32>,
}

impl RecentCounter {

    fn new() -> Self {
        Self {
            q: VecDeque::new(),
        }
    }
    
    fn ping(&mut self, t: i32) -> i32 {
        while let Some(&val) = self.q.front() {
            if val >= t - 3000 {
                break;
            }
            self.q.pop_front();
        }
        self.q.push_back(t);
        self.q.len() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */