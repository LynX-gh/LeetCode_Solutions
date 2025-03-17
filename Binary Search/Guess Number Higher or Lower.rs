/** 
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut l = 1;
        let mut r = n;

        while l <= r {
            let mid = l + (r - l) / 2;
            let res = guess(mid);
            if res == 0 {
                return mid;
            } else if res == 1 {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        -1
    }
}