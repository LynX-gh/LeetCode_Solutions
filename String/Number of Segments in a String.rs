impl Solution {
    pub fn count_segments(s: String) -> i32 {
        s.split_ascii_whitespace().count() as i32
    }
}
