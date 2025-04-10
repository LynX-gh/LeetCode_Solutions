impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let mut res = i32::MAX;

        for i in 0..nums.len() {
            let mut l = i+1;
            let mut r = nums.len() - 1;
            while l < r {
                let mut cur_sum = nums[i] + nums[l] + nums[r];
                if (cur_sum - target).abs() < (res - target).abs() {
                    res = cur_sum;
                }
                if cur_sum < target {
                    l += 1;
                } else if cur_sum > target {
                    r -= 1;
                } else {
                    return cur_sum;
                }
            }
        }
        res
    }
}
