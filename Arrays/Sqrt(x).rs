impl Solution {
    pub fn my_sqrt(mut x: i32) -> i32 {
        let x = x as i64;
        let mut left = 0;
        let mut right = x / 2 + 1;

        while left <= right {
            let mid = left + (right-left)/2;

            if mid * mid == x {
                return mid as i32;
            } else if mid * mid > x {
                right = mid-1;
            } else {
                left = mid+1;
            }
        }
        right as i32
    }
}