impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        fn dfs(
            node: usize,
            parent: isize,
            depth: usize,
            children: &Vec<Vec<usize>>,
            color: &mut Vec<usize>,
        ) -> i32 {
            let mut res = 1 - (depth % 2) as i32;
            color[node] = depth % 2;
            for &child in &children[node] {
                if child as isize == parent {
                    continue;
                }
                res += dfs(child, node as isize, depth + 1, children, color);
            }
            res
        }

        fn build(edges: &Vec<Vec<i32>>, color: &mut Vec<usize>) -> Vec<i32> {
            let n = edges.len() + 1;
            let mut children = vec![vec![]; n];
            for edge in edges {
                let u = edge[0] as usize;
                let v = edge[1] as usize;
                children[u].push(v);
                children[v].push(u);
            }
            let res = dfs(0, -1, 0, &children, color);
            vec![res, (n as i32) - res]
        }

        let n = edges1.len() + 1;
        let m = edges2.len() + 1;
        let mut color1 = vec![0; n];
        let mut color2 = vec![0; m];
        let count1 = build(&edges1, &mut color1);
        let count2 = build(&edges2, &mut color2);
        let mut res = vec![0; n];
        for i in 0..n {
            res[i] = count1[color1[i]] + count2[0].max(count2[1]);
        }
        res
    }
}