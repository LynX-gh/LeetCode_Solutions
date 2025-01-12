impl Solution {
    pub fn reverse_words(s: String) -> String {
        let words: Vec<&str> = s.split_whitespace().collect();
        words.into_iter().rev().collect::<Vec<&str>>().join(" ")
    }
}