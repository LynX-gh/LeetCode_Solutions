impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let max = *nums.iter().max().unwrap();
        let mut j = 0;
        let mut res = 0;
        let mut freq = 0;

        for i in 0..nums.len() {
            if nums[i] == max {
                freq += 1;
            }

            while freq >= k {
                res += (nums.len() - i) as i64;
                if nums[j] == max {
                    freq -= 1;
                }
                j += 1;
            }
        }
        res
    }
}