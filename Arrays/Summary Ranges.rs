impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.len() == 0  {
            return vec![];
        }

        let mut range_start = nums[0];
        let mut range_curr = nums[0];
        let mut res = Vec::with_capacity(nums.len());

        for i in 1..nums.len() {
            if nums[i] != range_curr + 1 {
                if range_start == range_curr {
                    res.push(format!("{}", range_start));
                }
                else {
                    res.push(format!("{}->{}", range_start, range_curr))
                }
                range_start = nums[i];
                range_curr = nums[i];
            }
            else {
                range_curr += 1;
            }
        }
        if range_start == range_curr {
            res.push(format!("{}", range_start));
        }
        else {
            res.push(format!("{}->{}", range_start, range_curr))
        }
        res
    }
}