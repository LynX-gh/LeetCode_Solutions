impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let nums:Vec<i32> = (1..=n).collect();

        Self::backtrack(0, k, &mut vec![], &nums, &mut res);
        res
    }

    fn backtrack(start: usize, max_depth: i32, prev: &mut Vec<i32>, nums: &Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if prev.len() as i32 == max_depth {
            res.push(prev.clone());
            return;
        }

        for i in start..nums.len() {
            prev.push(nums[i]);
            Self::backtrack(i+1, max_depth, prev, nums, res);
            prev.pop();
        }
    }
}