use std::collections::HashSet;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        candidates.sort();

        Self::backtrack(&mut res, 0, target, &candidates, &mut vec![]);

        res
    }

    fn backtrack(res: &mut Vec<Vec<i32>>, start: usize, target: i32, nums: &Vec<i32>, prev: &mut Vec<i32>) {
        let mut seen = HashSet::with_capacity(nums.len() - start);
        for i in start..nums.len() {
            let new_target = target - nums[i];

            if seen.insert(nums[i]) {
                if new_target == 0 {
                    prev.push(nums[i]);
                    res.push(prev.clone());
                    prev.pop();
                } else if new_target > 0 {
                    prev.push(nums[i]);
                    Self::backtrack(res, i+1, new_target, nums, prev);
                    prev.pop();
                } else {
                    return;
                }
            }
        }
    }
}