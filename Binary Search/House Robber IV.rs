impl Solution {
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        let mut min = *nums.iter().min().unwrap();
        let mut max = *nums.iter().max().unwrap();
        let mut res = nums[0];

        while min <= max {
            let mid = min + (max - min) / 2;

            if Self::is_valid(mid, &nums, &k) {
                res = mid;
                max = mid - 1;
            } else {
                min = mid + 1;
            }
        }
        res
    }

    fn is_valid(cap: i32, nums: &Vec<i32>, k: &i32) -> bool {
        let mut i = 0;
        let mut res = 0;

        while i < nums.len() {
            if nums[i] <= cap {
                res += 1;
                i += 1;
            }
            i += 1;
        }
        res >= *k
    }
}