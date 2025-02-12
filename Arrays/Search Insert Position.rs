impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len()-1;

        while left <= right && right < nums.len() {
            let mid = (left + right) / 2;

            if nums[mid] == target {
                return mid as i32;
            }
            else if nums[mid] > target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        left as i32
    }
}