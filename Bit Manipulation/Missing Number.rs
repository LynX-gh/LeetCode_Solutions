impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 1..=nums.len() as i32 {
            res ^= i;
        }
        for n in nums {
            res ^= n;
        }
        res
    }
}