// O(log n)
impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let neg_count = Self::binary_search(&nums, 0) as i32;
        let pos_count = (nums.len() - Self::binary_search(&nums, 1)) as i32;
        if neg_count > pos_count { neg_count } else { pos_count }
    }

    fn binary_search(nums: &Vec<i32>, target: i32) -> usize {
        let (mut left, mut right, mut result) = (0, nums.len() as isize - 1, nums.len());
        
        while left <= right {
            let mid = ((left + right) / 2) as usize;
            if nums[mid] < target {
                left = mid as isize + 1;
            } else {
                result = mid;
                right = mid as isize - 1;
            }
        }
        
        result
    }
}

// O(n)
impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let mut pos = 0;
        let mut neg = 0;

        for num in nums {
            if num > 0 {
                pos += 1;
            } else if num < 0 {
                neg += 1;
            }
        }
        pos.max(neg)
    }
}