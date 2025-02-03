use std::collections::HashSet;

// Topological Sort BFS - Kahns Algo

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let mut adj_list = vec![vec![]; num_courses];
        let mut in_degree = vec![0; num_courses];

        for prereq in prerequisites {
            let a = prereq[0] as usize;
            let b = prereq[1] as usize;
            adj_list[b].push(a);
            in_degree[a] +=1;
        }

        let mut stack: Vec<usize> = in_degree.iter().enumerate().flat_map(|(course, &count)| {
            if count == 0 {
                Some(course)
            } else {
                None
            }
        }).collect();


        let mut counter = 0;
        while let Some(node) = stack.pop() {
            counter += 1;
            for &neighbor in &adj_list[node] {
                let deg = &mut in_degree[neighbor];
                *deg -= 1;
                if *deg == 0 {
                    stack.push(neighbor);
                }
            }
        }
        counter == num_courses 
    }
}

// Topological Sort DFS

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut adj = vec![vec![]; num_courses as usize];
        let mut visiting = HashSet::new();
        let mut visited = HashSet::new();

        for preq in prerequisites {
            adj[preq[0] as usize].push(preq[1] as usize);
        }

        for course in 0..num_courses as usize {
            if !Self::dfs(course, &adj, &mut visiting, &mut visited) {
                return false;
            }
        }

        true
    }

    fn dfs(node: usize, adj: &Vec<Vec<usize>>, visiting: &mut HashSet<usize>, visited: &mut HashSet<usize>) -> bool {
        if visiting.contains(&node) {
            return false;   // Cycle in Graph
        }
        if visited.contains(&node) {
            return true;
        }

        visiting.insert(node);
        for &neighbor in &adj[node] {
            if !Self::dfs(neighbor, adj, visiting, visited) {
                return false;
            }
        }
        visiting.remove(&node);
        visited.insert(node);

        true
    }
}