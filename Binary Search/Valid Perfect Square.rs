impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut l = 0;
        let mut r = num / 2 + 1;

        while l <= r {
            let mid = l + (r - l) / 2;
            let sq = mid as i64 * mid as i64;
            if sq == num as i64 {
                return true;
            } else if sq < num as i64 {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        false
    }
}