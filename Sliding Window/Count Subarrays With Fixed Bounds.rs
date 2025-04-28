impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let (mut ans, mut bad, mut left, mut right) = (0i64, -1, -1, -1);

        for (i, num) in nums.into_iter().enumerate() {
            let i = i as i32;

            if num < min_k || num > max_k {
                bad = i;
            }
            
            if num == min_k{
                left = i;
            }
            
            if num == max_k{
                right = i;
            }
            
            ans += std::cmp::max(0, std::cmp::min(right, left) - bad) as i64;
        }

        ans
    }
}