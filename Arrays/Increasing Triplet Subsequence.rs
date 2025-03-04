impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut res = 0;
        let mut stk = vec![];

        for &num in nums.iter() {
            if let Err(idx) = stk.binary_search(&num) {
                if idx == stk.len() {
                    stk.push(num);
                    if stk.len() == 3 {
                        return true;
                    }
                } else {
                    stk[idx] = num;
                }
            }
        }
        false
    }
}