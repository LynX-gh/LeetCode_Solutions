impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut res = 0;
        let mut val_ptr = 0;

        for i in 0..nums.len() {
            if nums[i] != val {
                nums.swap(val_ptr, i);
                val_ptr += 1;
                res += 1;
            }
        }
        res
    }
}