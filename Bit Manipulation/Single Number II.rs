impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut x1 = 0;
        let mut x2 = 0;
        let mut mask = 0;

        for num in nums {
            x2 ^= x1 & num;
            x1 ^= num;
            mask = !(x1 & x2);
            x2 &= mask;
            x1 &= mask;
        }

        x1 | x2
    }
}