use std::collections::HashSet;

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut seen = HashSet::new();

        for i in 0..is_connected.len() {
            let mut province = 0;
            if !seen.contains(&i) {
                Self::dfs(&is_connected, &mut province, &mut seen, i);
            }
            if province > 0 {
                res += 1;
            }
        }
        res
    }

    fn dfs(edges: &Vec<Vec<i32>>, province: &mut i32, seen: &mut HashSet<usize>, n: usize) {
        if seen.insert(n) {
            *province += 1;
            for j in 0..edges[n].len() {
                if edges[n][j] == 1 {
                    Self::dfs(edges, province, seen, j);
                }
            }
        }
    }
}