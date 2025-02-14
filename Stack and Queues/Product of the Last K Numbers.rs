/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

#[derive(Debug, Clone, Default)]
struct ProductOfNumbers {
    nums: Vec<u64>,
}

impl ProductOfNumbers {
    fn new() -> Self {
        Default::default()
    }

    fn add(&mut self, num: i32) {
        if num == 0 {
            self.nums.clear();
        } else {
            match self.nums.last() {
                None | Some(&0) => self.nums.push(num as u64),
                Some(v) => self.nums.push(num as u64 * v),
            };
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        let (n, k) = (self.nums.len(), k as usize);
        let Some(i) = n.checked_sub(k) else {
            return 0;
        };
        if i == 0 {
            self.nums[n - 1] as _
        } else {
            (self.nums[n - 1] / self.nums[i - 1]) as _
        }
    }
}

/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * let obj = ProductOfNumbers::new();
 * obj.add(num);
 * let ret_2: i32 = obj.get_product(k);
 */