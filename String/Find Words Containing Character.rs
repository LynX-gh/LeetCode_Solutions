impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        let mut res = Vec::with_capacity(words.len());

        for i in 0..words.len() {
            for c in words[i].chars() {
                if c == x {
                    res.push(i as i32);
                    break;
                }
            }
        }
        res
    }
}