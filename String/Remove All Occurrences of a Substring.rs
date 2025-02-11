impl Solution {
    pub fn remove_occurrences(mut s: String, part: String) -> String {
        while let Some(idx) = s.find(&part) {
            s.drain(idx..(idx+part.len()));
        }
        s
    }
}