impl Solution {
    pub fn minimum_pair_removal(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }
        
        let mut res = 0;
        let mut changed = true;
        let mut min_idx = 0;
        let mut min_sum = 0;

        loop {
            min_idx = 0;
            min_sum = i32::MAX;
            if Self::check_sort(&nums) {
                return res;
            }
            
            for i in 1..nums.len() {
                if nums[i] + nums[i-1] < min_sum {
                    min_sum = nums[i] + nums[i-1];
                    min_idx = i;
                }
            }
            nums[min_idx-1] += nums[min_idx];
            nums.remove(min_idx);
            res += 1;
        }
        res
    }

    fn check_sort(nums: &Vec<i32>) -> bool {
        for i in 0..nums.len() - 1 {
            if nums[i] > nums[i+1] {
                return false;
            }
        }
        true
    }
}
