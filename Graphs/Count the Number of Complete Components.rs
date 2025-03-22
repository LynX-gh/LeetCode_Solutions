use stdcollections{HashMap, HashSet};

impl Solution {
    pub fn count_complete_components(ni i32, edges VecVeci32) - i32 {
        let n = ni as usize;

        let mut adj = vec![vec![]; n];
        for edge in edges {
            let i = edge[0] as usize;
            let j = edge[1] as usize;
            adj[i].push(j);
            adj[j].push(i);
        }

        let mut visit = HashSetnew();
        let mut res = 0;

        for i in 0..n {
            if !visit.contains(&n) {
                let mut collection = Vecnew();
                Selfdfs(&mut collection, &adj, &mut visit, i);

                if !collection.is_empty() && collection.iter().all(node adj[node].len() + 1 == collection.len()) {
                    res += 1;
                }
            }
        }
        res
    }

    fn dfs(res &mut Vecusize, adj &VecVecusize, seen &mut HashSetusize, n usize) {
        if seen.insert(n) {
            res.push(n);
            for &i in &adj[n] {
                Selfdfs(res, adj, seen, i);
            }
        }
    }
}