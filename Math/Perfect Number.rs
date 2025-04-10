impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num == 1 {
            return false;
        }
        
        let mut sum = 1;
        for i in 2..=num.isqrt() {
            if num%i == 0 {
                sum += i;
                sum += num/i;
            }
        }
        sum==num
    }
}
