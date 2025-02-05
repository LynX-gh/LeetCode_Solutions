use std::iter::zip;

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let mut sw = [0, 0];
        
        for (s1_ch, s2_ch) in zip(s1.as_bytes(), s2.as_bytes()) {
            if s1_ch != s2_ch {
                if sw[0] == 0 {
                    sw[0] = *s1_ch;
                    sw[1] = *s2_ch;
                } else if sw[0] != 1 && sw[0] == *s2_ch && sw[1] == *s1_ch {
                    sw[0] = 1;
                } else {
                    return false;
                }
            }
        }
        sw[0] == 1 || sw[0] == 0
    }
}