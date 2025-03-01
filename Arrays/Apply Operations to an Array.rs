impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len()];
        let mut res_idx = 0;

        for i in 0..nums.len() {
            if i + 1 < nums.len() && nums[i] == nums[i+1] {
                nums[i] = nums[i] * 2;
                nums[i+1] = 0;
            }
            if nums[i] != 0 {
                res[res_idx] = nums[i];
                res_idx += 1;
            }
        }
        res
    }
}