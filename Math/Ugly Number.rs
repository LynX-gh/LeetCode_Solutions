impl Solution {
    pub fn is_ugly(mut n: i32) -> bool {
        for i in 2..6 {
            while n > 1 && n % i == 0 {
                n /= i;
            }
        }
        n == 1
    }
}