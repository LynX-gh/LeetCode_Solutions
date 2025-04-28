impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let mut presum = vec![0; nums.len() + 1];
        let mut res = 0;

        for i in 0..nums.len() {
            presum[i+1] = nums[i] as i64 + presum[i];
        }

        let mut i = 0;
        for j in 0..nums.len() {
            while i <= j && (presum[j+1] - presum[i]) * (j - i + 1) as i64 >= k {
                i += 1;
            }
            res += (j - i + 1) as i64;
        }
        res
    }
}