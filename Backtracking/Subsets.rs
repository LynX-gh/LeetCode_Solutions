impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        Self::backtrack(&nums, 0, &mut res, &mut vec![]);
        res
    }

    fn backtrack(nums: &Vec<i32>, start: usize, res: &mut Vec<Vec<i32>>, prev: &mut Vec<i32>) {
        res.push(prev.clone());

        for i in start..nums.len() {
            prev.push(nums[i]);
            Self::backtrack(nums, i+1, res, prev);
            prev.pop();
        }

    }
}