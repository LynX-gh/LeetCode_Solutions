// Kahns Topological Sort with Queue
use std::collections::VecDeque;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adj = vec![vec![]; num_courses as usize];
        let mut indeg = vec![0; num_courses as usize];

        for prereq in prerequisites {
            adj[prereq[1] as usize].push(prereq[0] as usize);
            indeg[prereq[0] as usize] += 1;
        }

        let mut q = VecDeque::new();

        for i in 0..num_courses as usize {
            if indeg[i] == 0 {
                q.push_back(i);
            }
        }
        
        let mut res = Vec::new();
        while let Some(node) = q.pop_back() {
            res.push(node as i32);
            for &prereq in &adj[node] {
                indeg[prereq] -= 1;
                if indeg[prereq] == 0 {
                    q.push_front(prereq);
                }
            }
        }

        if res.len() == num_courses as usize {
            res
        } else {
            vec![]
        }
    }
}

// Kahns Topological Sort with Stack
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adj = vec![vec![]; num_courses as usize];
        let mut indeg = vec![0; num_courses as usize];

        for preq in prerequisites {
            adj[preq[1] as usize].push(preq[0] as usize);
            indeg[preq[0] as usize] += 1;
        }
        
        let mut stack = Vec::new();
        for i in 0..num_courses as usize {
            if indeg[i] == 0 {
                stack.push(i);
            }
        }

        let mut res = Vec::new();
        while let Some(node) = stack.pop() {
            res.push(node as i32);
            for &preq in &adj[node] {
                indeg[preq] -= 1;
                if indeg[preq] == 0 {
                    stack.push(preq);
                }
            }
        }
        
        if res.len() == num_courses as usize {
            res
        } else {
            vec![]
        }
    }
}