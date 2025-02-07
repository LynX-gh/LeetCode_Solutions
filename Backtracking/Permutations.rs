impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];

        for i in 0..nums.len() {
            Self::backtrack(&nums, &mut res, &mut vec![nums[i]]);
        }
        res
    }

    fn backtrack(nums: &Vec<i32>, res: &mut Vec<Vec<i32>>, prev: &mut Vec<i32>) {
        if prev.len() == nums.len() {
            res.push(prev.to_vec());
        } else {
            for i in 0..nums.len() {
                if !prev.contains(&nums[i]) {
                    prev.push(nums[i]);
                    Self::backtrack(nums, res, prev);
                    prev.pop();
                }
            }
        }
    }
}