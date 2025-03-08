impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut stk = Vec::new();

        for ch in s.chars() {
            if ch == '*' {
                stk.pop();
            } else {
                stk.push(ch);
            }
        }

        String::from_iter(stk)
    }
}