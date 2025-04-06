impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut gp = g.len() - 1;
        let mut sp = s.len() - 1;
        
        g.sort();
        s.sort();
        while gp < g.len() && sp < s.len() {
            if s[sp] >= g[gp] {
                res += 1;
                sp -= 1;
            }
            gp -= 1;
        }
        res
    }
}
