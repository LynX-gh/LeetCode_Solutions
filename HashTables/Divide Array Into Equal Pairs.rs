impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut pairs = vec![false; 501];
        for num in nums {
            pairs[num as usize] = !pairs[num as usize];
        }
        for pair in pairs {
            if pair {
                return false;
            }
        }
        true
    }
}