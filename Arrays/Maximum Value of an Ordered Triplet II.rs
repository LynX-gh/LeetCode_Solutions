impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut premax = vec![0; nums.len()+1];
        let mut postmax = vec![0; nums.len()+1];

        for i in 1..nums.len() {
            premax[i] = nums[i-1].max(premax[i-1]);
        }

        for i in (0..nums.len()-1).rev() {
            postmax[i] = nums[i+1].max(postmax[i+1]);
        }
        
        let mut res = 0;
        for i in 1..nums.len()-1 {
            if premax[i] > nums[i] && (premax[i] - nums[i]) as i64 * postmax[i] as i64 > res {
                res = (premax[i] - nums[i]) as i64 * postmax[i] as i64;
            }
        }
        res
    }
}
