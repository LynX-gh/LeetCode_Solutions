impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut res = vec![];
        let mut line = vec![];
        let mut line_len = 0;

        for word in words.into_iter() {
            if word.len() as i32 + line.len() as i32 + line_len > max_width {
                for i in 0..(max_width - line_len) {
                    let idx = i as usize % (if line.len() > 1 { line.len() - 1 } else { line.len() });
                    line[idx] = format!("{} ", line[idx]);
                }
                res.push(line.join(""));
                line.clear();
                line_len = 0;
            }
            line_len += word.len() as i32;
            line.push(word);
        }

        res.push(format!("{:<width$}", line.join(" "), width=max_width as usize));
        res
    }
}