// O(1)

impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        n * (n + 1) / 2 - (n/m) * ((n/m) + 1) * m
    }
}

// O(n)

impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        let mut res = 0;
        for i in 1..=n {
            if i % m == 0 {
                res -= i; 
            } else {
                res += i;
            }
        }
        res
    }
}