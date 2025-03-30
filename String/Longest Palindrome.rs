impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut pairs = 0;
        let mut solo = 0;
        let a_val = 'a' as u8;
        let mut ch_map = vec![0; 300];

        for ch in s.chars() {
            ch_map[(ch as u8 - a_val) as usize] += 1;
            if ch_map[(ch as u8 - a_val) as usize] %2 == 0 {
                pairs += 2;
                solo -= 1;
            } else {
                solo += 1;
            }
        }

        if solo > 0 {
            pairs + 1
        } else {
            pairs
        }
    }
}
