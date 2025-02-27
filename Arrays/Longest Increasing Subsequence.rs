// Greedy with Binary Search O(n logn)

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut sis = vec![];

        for i in 0..nums.len() {
            if let Err(idx) = sis.binary_search(&nums[i]) {
                if idx == sis.len() {
                    sis.push(nums[i]);
                } else {
                    sis[idx] = nums[i];
                }
            }
        }

        sis.len() as i32
    }
}