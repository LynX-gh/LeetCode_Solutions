impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut rc = 0;
        let mut bc = 0;

        for &num in nums.iter() {
            if num == 0 {
                rc += 1;
            } else if num == 1 {
                bc += 1;
            }
        }
        for i in 0..rc {
            nums[i] = 0;
        }
        for i in rc..rc+bc {
            nums[i] = 1;
        }
        for i in rc+bc..nums.len() {
            nums[i] = 2;
        }
    }
}