impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let mut res = 1;
        
        for i in 0..nums.len() {
            let mut first = nums[i];
            for j in i+1..nums.len() {
                if (first & nums[j] != 0) {
                    break;
                }
                first ^= nums[j];
                res = res.max((j - i + 1) as i32);
            }
        }
        res
    }
}