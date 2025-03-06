impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut rsum = vec![0; nums.len()];
        
        for i in (1..nums.len()).rev() {
            rsum[i-1] = nums[i] + rsum[i];
        }

        let mut lsum = 0;

        for i in 0..nums.len() {
            if lsum == rsum[i] {
                return i as i32;
            }
            lsum += nums[i];
        }
        -1
    }
}