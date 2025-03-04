impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut last = chars[0];
        let mut curr = 1;
        let mut last_num = 1;

        while curr < chars.len() {
            if chars[curr] == last {
                last_num += 1;
                chars.remove(curr);
            } else {
                if last_num > 1 {
                    for ch in last_num.to_string().chars() {
                        chars.insert(curr, ch);
                        curr+=1;
                    }
                }
                last = chars[curr];
                last_num = 1;
                curr+=1;
            }
        }
        if last_num > 1 {
            for ch in last_num.to_string().chars() {
                chars.push(ch);
            }
        }
        chars.len() as i32
    }
}