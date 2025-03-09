impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        // O n^2 (Check each subarray)
        // let mut sptr = 0;
        // let mut eptr = 0;

        // let mut window_len = 0;

        // while window_len < nums.len() {
        //     sptr = 0;
        //     eptr = 0 + window_len;
        //     while eptr < nums.len() {
        //         let mut sum = 0;
        //         for i in sptr..=eptr {
        //             sum += nums[i];
        //         }
        //         // println!("{sptr} = {eptr} = {sum}");
        //         if sum >= target {
        //             return (eptr - sptr + 1) as i32;
        //         }
        //         sptr += 1;
        //         eptr += 1;
        //     }
        //     window_len += 1;
        // }
        // 0

        // O n (Sliding Window)
        use std::cmp;

        let mut sptr = 0;
        let mut eptr = 0;
        let mut sum = 0;
        let mut window_len = usize::MAX;

        while eptr < nums.len() && sptr < nums.len() {
            sum += nums[eptr];
            while sum >= target {
                window_len = cmp::min(window_len, eptr - sptr + 1);
                sum -= nums[sptr];
                sptr += 1;
            }
            eptr += 1;
        }
        match window_len {
            usize::MAX => 0,
            _ => window_len as i32
        }
    }
}