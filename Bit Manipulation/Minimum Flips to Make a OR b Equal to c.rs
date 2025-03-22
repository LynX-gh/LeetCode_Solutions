impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let bits_needed = (a | b) ^ c;
        bits_needed.count_ones() as i32 + (a & b & bits_needed).count_ones() as i32
    }
}