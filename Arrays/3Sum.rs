impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut sorted_nums = nums;
        sorted_nums.sort();

        for i in 0..sorted_nums.len() - 2 {
            // If all nums after i are positive sum cant be 0
            if sorted_nums[0] > 0 {
                break;
            }

            // Remove duplicate entries
            if i > 0 && sorted_nums[i-1] == sorted_nums[i] {
                continue;
            }

            let mut ptr1 = i+1;
            let mut ptr2 = sorted_nums.len() - 1;

            while ptr1 < ptr2 {
                // println!("{}", i);
                let sum = sorted_nums[ptr1] + sorted_nums[ptr2] + sorted_nums[i];

                if  sum == 0 {
                    res.push(vec!(sorted_nums[i], sorted_nums[ptr1], sorted_nums[ptr2]));
                    ptr2 -= 1;
                    while ptr1 < ptr2 && sorted_nums[ptr2] == sorted_nums[ptr2 + 1]{ptr2 -= 1;}
                    ptr1 += 1;
                    while ptr1 < ptr2 && sorted_nums[ptr1] == sorted_nums[ptr1 - 1]{ptr1 += 1;}
                }
                else if sum > 0 {
                    ptr2 -= 1;
                }
                else {
                    ptr1 += 1;
                }
            }
        }
        res
    }
}