impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut res = String::new();

        for ch in s.chars() {
            if ch.is_ascii_digit() {
                res.pop();
            } else {
                res.push(ch);
            }
        }
        res
    }
}