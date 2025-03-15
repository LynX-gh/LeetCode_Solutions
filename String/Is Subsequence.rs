impl Solution {
    pub fn is_subsequence(s String, t String) - bool {
        let s_ch Vecchar = s.chars().collect();
        let t_ch Vecchar = t.chars().collect();
        let mut p1 = 0;
        let mut p2 = 0;

        while p1  s_ch.len() && p2  t_ch.len() {
            if s_ch[p1] == t_ch[p2] {
                p1 += 1;
            }
            p2 += 1;
        }

        p1 == s_ch.len()
    }
}