impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        candidates.sort();

        Self::backtrack_sum(0, &mut vec![], &mut res, &candidates, target);
        res
    }

    fn backtrack_sum(start: usize, prev: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, nums: &Vec<i32>, target: i32) {
        for i in start..nums.len() {

            let new_target = target - nums[i];
            
            if new_target == 0 {
                prev.push(nums[i]);
                res.push(prev.clone());
                prev.pop();
                break;
            } else if new_target > 0 {
                prev.push(nums[i]);
                Self::backtrack_sum(i, prev, res, nums, new_target);
                prev.pop();
            } else {
                break;
            }
        }
    }
}