impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        for ch in s.chars() {
            match ch {
                '(' => stack.push(')'),
                '{' => stack.push('}'),
                '[' => stack.push(']'),
                ')' | '}' | ']' => {
                    match stack.pop() {
                        Some(x) if x == ch => (),
                        _ => return false,
                    };
                },
                _ => return false,
            }
        }
        stack.is_empty()
    }
}