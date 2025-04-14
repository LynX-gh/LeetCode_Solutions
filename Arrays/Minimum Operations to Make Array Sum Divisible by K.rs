impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let x: i32 = nums.iter().sum();
        x % k
    }
}
