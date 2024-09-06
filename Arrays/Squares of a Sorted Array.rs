impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 2 {
            return vec!(nums[0].pow(2));
        }

        let mut res = Vec::with_capacity(nums.len());

        let mut startptr:isize = 0;
        let mut endptr:isize = (nums.len() - 1).try_into().unwrap();

        while startptr <= endptr {
            if nums[startptr as usize].abs() >= nums[endptr as usize].abs() {
                res.push(nums[startptr as usize].pow(2));
                startptr += 1;
            }
            else {
                res.push(nums[endptr as usize].pow(2));
                endptr -= 1;
            }
        }

        res.reverse();
        res
    }
}

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 2 {
            return vec!(nums[0].pow(2));
        }

        let mut res = Vec::with_capacity(nums.len());

        let mut midptr = 0;
        for num in nums.iter() {
            if *num >= 0 {break;}
            else {midptr += 1;}
        }
        let mut negptr:isize = midptr-1;
        let mut posptr:isize = midptr;
        while negptr >= 0 && posptr < nums.len() as isize{
            if nums[negptr as usize].abs() > nums[posptr as usize] {
                res.push(nums[posptr as usize].pow(2));
                posptr += 1;
            }
            else {
                res.push(nums[negptr as usize].pow(2));
                negptr -= 1;
            }
        }
        while negptr >= 0 {
            res.push(nums[negptr as usize].pow(2));
            negptr -= 1;
        }
        while posptr < nums.len() as isize { 
            res.push(nums[posptr as usize].pow(2));
            posptr += 1;
        }

        res.reverse();
        res
    }
}
