use std::collections::HashSet;

impl Solution {
    pub fn number_of_components(properties: Vec<Vec<i32>>, kl: i32) -> i32 {
        let k = kl as usize;
        let n = properties.len();
        let mut adj = vec![vec![]; n];

        let mut sets: Vec<HashSet<i32>> = Vec::new();
        for property in properties {
            sets.push(HashSet::from_iter(property));
        }
        for i in 0..n {
            for j in 0..n {
                if i != j && sets[i].intersection(&sets[j]).count() >= k {
                    adj[i].push(j);
                }
            }
        }

        let mut res = 0;
        let mut seen = HashSet::new();
        for i in 0..n {
            if !seen.contains(&i) {
                Self::dfs(&adj, &mut seen, i);
                res += 1;
            }
        }
        res
    }

    fn dfs(adj: &Vec<Vec<usize>>, seen: &mut HashSet<usize>, n: usize) {
        if seen.insert(n) {
            for &i in &adj[n] {
                Self::dfs(adj, seen, i);
            }
        }
    }
}