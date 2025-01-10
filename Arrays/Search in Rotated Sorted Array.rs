impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let res = nums.iter().position(|n| *n == target);
        match res {
            Some(x) => return x as i32,
            None => return -1
        };
    }
}