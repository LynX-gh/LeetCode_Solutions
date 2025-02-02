impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut rot_flag = 0;

        for i in 1..n {
            if nums[i] < nums[i-1] {
                rot_flag += 1;
                if rot_flag > 1{
                    return false;
                }
            }
        }
        (rot_flag == 1 && nums[n-1] <= nums[0]) || rot_flag == 0
    }
}