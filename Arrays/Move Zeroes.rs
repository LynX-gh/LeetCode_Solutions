impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }

        let mut zero_ptr = 0;

        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(zero_ptr, i);
                zero_ptr += 1;
            }
        }
    }
}