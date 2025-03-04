impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![1; nums.len()];
        let mut post = 1;

        for i in (0..nums.len()).rev() {
            res[i] = post;
            post *= nums[i]
        }

        let mut pre = 1;
        for i in (0..nums.len()) {
            res[i] *= pre;
            pre *= nums[i];
        }

        res
    }
}