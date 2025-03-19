impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..nums.len()-2 {
            if nums[i] == 0 {
                res += 1;
                nums[i] = 1;
                if nums[i+1] == 1 {
                    nums[i+1] = 0;
                } else {
                    nums[i+1] = 1;
                }
                if nums[i+2] == 1 {
                    nums[i+2] = 0;
                } else {
                    nums[i+2] = 1;
                }
            }
        }
        if nums.iter().all(|&x| x == 1) {
            res
        } else {
            -1
        }
    }
}