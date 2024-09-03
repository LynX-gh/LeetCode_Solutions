/* impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::with_capacity(nums.len());
        for num in nums {
            res.push(match res.last() {
                Some(x) => x+num,
                None => num
            })
        }
        return res
    }
} */

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::with_capacity(nums.len());
        let mut lastVal:i32 = 0;
        for num in nums {
            lastVal = num + lastVal;
            res.push(lastVal);
        }
        return res
    }
}