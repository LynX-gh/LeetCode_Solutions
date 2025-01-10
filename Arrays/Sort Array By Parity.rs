impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut sorted_nums = nums;

        if sorted_nums.len() < 1 {
            return sorted_nums;
        }

        let mut odd_ptr = 0;

        for i in 0..sorted_nums.len() {
            if sorted_nums[i] % 2 == 0 {
                sorted_nums.swap(odd_ptr, i);
                odd_ptr += 1;
            }
        }
        return sorted_nums;
    }
}