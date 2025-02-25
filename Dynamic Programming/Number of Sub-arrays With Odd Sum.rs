impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let mut res = 0_i64;
        let mut sum = 0;
        let mut prev_odd = 0;
        let mut prev_even = 0;

        for i in 0..arr.len() {
            sum += arr[i];
            if sum & 1 == 1 {
                prev_odd += 1;
                res += prev_even + 1;
            } else {
                prev_even += 1;
                res += prev_odd;
            }
        }
        (res % 1000000007) as i32
    }
}