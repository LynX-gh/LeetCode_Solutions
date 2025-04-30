// One Liner
impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, x| if (x.ilog10()+1) % 2 == 0 {acc + 1} else {acc})
    }
}

// O(n)
impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for num in nums {
            if (num.ilog10() + 1) % 2 == 0 {
                res += 1;
            }
        }
        res
    }
}