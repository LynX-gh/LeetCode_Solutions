impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut lz = -1;
        let mut res = 0;
        
        for i in 0..nums.len() {
            if nums[i] == 0 {
                if lz == -1 {
                    lz = i as i32;
                } else {
                    start = lz as usize + 1;
                    lz = i as i32;
                }
            }
            res = res.max(i - start);
        }
        res as i32
    }
}