/* impl Solution {
    pub fn number_of_steps(mut num: i32) -> i32 {
        let mut res = 0;
        while num > 0 {
            if num % 2 == 0 {
                res += 1;
                num /= 2;
            }
            else {
                res += 1;
                num -= 1;
            }
        }
        res
    }
} */

impl Solution {
    pub fn number_of_steps(mut num: i32) -> i32 {
        if num == 0 { return 0; }
        let mut res = num & 1;
        while num > 0 {
            num >>= 1;
            res += 1;
            res += num & 1;
        }
        res - 1
    }
}