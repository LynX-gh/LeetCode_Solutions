use std::collections::HashSet;

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        nums.sort_unstable();
        Self::backtrack(0, &mut vec![], &nums, &mut res);
        res
    }

    fn backtrack(start:usize, prev: &mut Vec<i32>, nums: &Vec<i32>, res: &mut Vec<Vec<i32>>) {
        res.push(prev.clone());

        let mut seen = HashSet::with_capacity(nums.len() - start);
        for i in start..nums.len() {
            if seen.insert(nums[i]) {
                prev.push(nums[i]);
                Self::backtrack(i+1, prev, nums, res);
                prev.pop();
            }
        }
    }
}