impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut curr = String::from("1");
        for _ in 2..=n {
            curr = Self::rle(&curr);
        }
        curr
    }

    fn rle(s: &String) -> String {
        let mut res = String::new();

        let mut curr_char = s.chars().nth(0).unwrap();
        let mut curr_cnt = 0;

        for ch in s.chars() {
            if ch == curr_char {
                curr_cnt += 1;
            } else {
                res.push_str(&curr_cnt.to_string());
                res.push(curr_char);
                curr_char = ch;
                curr_cnt = 1;
            }
        }
        res.push_str(&curr_cnt.to_string());
        res.push(curr_char);
        res
    }
}