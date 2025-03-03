impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut res = String::new();
        let mut w1: Vec<char> = word1.chars().collect();
        let mut w2: Vec<char> = word2.chars().collect();
        let mut p1 = 0;
        let mut p2 = 0;

        while p1 < w1.len() && p2 < w2.len() {
            res.push(w1[p1]);
            p1 += 1;
            res.push(w2[p2]);
            p2 += 1;
        }

        while p1 < w1.len() {
            res.push(w1[p1]);
            p1 += 1;
        }

        while p2 < w2.len() {
            res.push(w2[p2]);
            p2 += 1;
        }

        res
    }
}