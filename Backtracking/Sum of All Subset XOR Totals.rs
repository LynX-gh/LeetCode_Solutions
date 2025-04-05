impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        Self::backtrack(&nums, 0, 0, &mut res);
        res
    }

    fn backtrack(nums: &Vec<i32>, idx: usize, curr: i32, res: &mut i32) {
        if idx == nums.len() {
            *res += curr;
            return;
        }

        Self::backtrack(nums, idx+1, curr^nums[idx], res);
        Self::backtrack(nums, idx+1, curr, res);
    }
}
