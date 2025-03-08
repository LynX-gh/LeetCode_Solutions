impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stk = Vec::new();
        let mut curr = String::new();
        let mut curr_n = 0;

        for ch in s.chars() {
            match ch {
                '[' => {
                    stk.push((curr.clone(), curr_n));
                    curr.clear();
                    curr_n = 0;
                },
                ']' => {
                    let (prev, prev_n) = stk.pop().unwrap();
                    curr = prev + &curr.repeat(prev_n);
                },
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
                    curr_n = curr_n * 10 + (ch as usize - 48);
                },
                _ => {
                    curr.push(ch);
                }
            }
        }
        curr
    }
}