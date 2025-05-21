impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let mut res = vec![0; nums.len()+1];

        for query in queries {
            res[query[0] as usize] += 1;
            res[query[1] as usize + 1] -= 1;
        }

        let mut presum = 0;
        for i in 0..nums.len() {
            presum += res[i];
            if presum < nums[i] {
                return false;
            }
        }
        true
    }
}