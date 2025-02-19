// DP O(n)
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut h1 = 0;
        let mut h2 = 0;

        for num in nums {
            let temp = h2.max(h1 + num);
            h1 = h2;
            h2 = temp;
        }

        h1.max(h2)
    }
}

// Memoization Bottom Up O(n)
use std::collections::HashMap;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut mem = HashMap::with_capacity(nums.len());
        Self::backtrack(0, &mut mem, &nums).max(Self::backtrack(1, &mut mem, &nums))
    }

    fn backtrack(index: usize, mem: &mut HashMap<usize, i32>, nums: &Vec<i32>) -> i32 {
        if index > nums.len() - 1{
            0
        } else if let Some(&val) = mem.get(&index) {
            val
        } else if index == nums.len() - 1 || index == nums.len() - 2 {
            mem.insert(index, nums[index]);
            nums[index]
        } else {
            let plus1 = Self::backtrack(index+2, mem, nums);
            let plus2 = Self::backtrack(index+3, mem, nums);
            mem.insert(index, plus1.max(plus2) + nums[index]);
            plus1.max(plus2) + nums[index]
        }
    }
}