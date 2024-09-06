impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        if nums.len() == 1 {
            if nums[0] == val {
                return 0;
            }
            else {
                return 1;
            }
        }

        let mut startptr = 0;
        let mut endptr = nums.len()-1;

        while startptr <= endptr && endptr < nums.len() {
            if nums[startptr] == val {
                nums[startptr] = nums[endptr];
                endptr -= 1;
            }
            else { 
                startptr += 1;
            }
        }
        (endptr + 1) as i32
    }
}