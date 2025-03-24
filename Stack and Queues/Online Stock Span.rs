struct StockSpanner {
    stk: Vec<(i32, i32)>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {

    fn new() -> Self {
        Self {
            stk: Vec::new(),
        }
    }
    
    fn next(&mut self, price: i32) -> i32 {
        let mut res = 1;
        while !self.stk.is_empty() && self.stk[self.stk.len()-1].0 <= price {
            res += self.stk[self.stk.len()-1].1;
            self.stk.pop();
        }
        self.stk.push((price, res));
        res
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */