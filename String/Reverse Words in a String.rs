impl Solution {
    pub fn reverse_words(s: String) -> String {
        let words: Vec<&str> = s.split_whitespace().collect();
        words.into_iter().rev().collect::<Vec<&str>>().join(" ")
    }
}

// One liner
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }
}