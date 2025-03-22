use std::collections::HashSet;

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut adj_dir = vec![vec![]; n as usize];
        let mut adj_und = vec![vec![]; n as usize];
        for connection in connections {
            let i = connection[0] as usize;
            let j = connection[1] as usize;
            adj_dir[i].push(j);
            adj_und[j].push(i);
        }

        let mut res = 0;
        Self::dfs(&mut res, &adj_dir, &adj_und, 0, 0);
        res
    }

    fn dfs(res: &mut i32, adj_dir: &Vec<Vec<usize>>, adj_und: &Vec<Vec<usize>>, node: usize, parent: usize) {
        for &next in &adj_dir[node] {
            if next != parent {
                *res += 1;
                Self::dfs(res, adj_dir, adj_und, next, node);
            }
        }

        for &next in &adj_und[node] {
            if next != parent {
                Self::dfs(res, adj_dir, adj_und, next, node);
            }
        }
    }
}