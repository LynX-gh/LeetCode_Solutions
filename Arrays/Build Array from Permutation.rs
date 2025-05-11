impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::with_capacity(nums.len());

        for i in 0..nums.len() {
            res.push(nums[nums[i] as usize]);
        }
        res
    }
}