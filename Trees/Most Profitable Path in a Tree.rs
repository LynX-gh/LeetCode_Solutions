use std::collections::VecDeque;

impl Solution {
    pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
        let n = amount.len();
        
        // Use an adjacency list instead of a matrix
        let mut adj = vec![vec![]; n];
        for edge in edges {
            adj[edge[0] as usize].push(edge[1] as usize);
            adj[edge[1] as usize].push(edge[0] as usize);
        }

        let mut bob_path = vec![-1; n];
        Self::dfs_bob(bob as usize, bob as usize, 0, &mut bob_path, &adj);

        let mut q = VecDeque::new();
        q.push_back((0, 0, usize::MAX, amount[0])); // (node, time, parent, profit)
        let mut res = i32::MIN;

        while let Some((node, time, parent, profit)) = q.pop_front() {
            let mut is_leaf = true;
            for &neighbor in &adj[node] {
                if neighbor != parent {
                    is_leaf = false;
                    let mut new_profit = amount[neighbor];
                    let new_time = time + 1;

                    if bob_path[neighbor] != -1 {
                        if new_time > bob_path[neighbor] {
                            new_profit = 0;
                        } else if new_time == bob_path[neighbor] {
                            new_profit /= 2;
                        }
                    }

                    q.push_back((neighbor, new_time, node, profit + new_profit));
                }
            }
            
            if is_leaf {
                res = res.max(profit);
            }
        }

        res
    }

    fn dfs_bob(src: usize, prev: usize, time: i32, path: &mut Vec<i32>, adj: &Vec<Vec<usize>>) -> bool {
        if src == 0 {
            path[src] = time;
            return true;
        }

        for &neighbor in &adj[src] {
            if neighbor == prev {
                continue;
            }
            if Self::dfs_bob(neighbor, src, time + 1, path, adj) {
                path[src] = time;
                return true;
            }
        }
        false
    }
}