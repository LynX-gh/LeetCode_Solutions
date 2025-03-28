impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut cnt = vec![0; 26];

        for ch in s.chars() {
            cnt[(ch as u8 - 97) as usize] += 1;
        }

        for ch in t.chars() {
            let idx = (ch as u8 - 97) as usize;
            cnt[idx] -= 1;
            if cnt[idx] == -1 {
                return (idx + 97) as u8 as char;
            }
        }

        'a'
    }
}