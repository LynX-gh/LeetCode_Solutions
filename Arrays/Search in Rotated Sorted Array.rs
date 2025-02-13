// O(log(n))

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid = (left + right) / 2;

            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] >= nums[left] {
                if nums[left] <= target && nums[mid] > target {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else {
                if nums[right] >= target && nums[mid] < target {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }
        -1
    }
}


// O(n)
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let res = nums.iter().position(|n| *n == target);
        match res {
            Some(x) => return x as i32,
            None => return -1
        };
    }
}