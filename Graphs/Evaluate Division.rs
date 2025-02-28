use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let graph = Self::builder(&equations, &values);
        queries.into_iter().map(|query| Self::dfs(&graph, query, &mut HashSet::new(), None).map_or(-1.0, |v| v)).collect()
    }

    fn builder(equations: &Vec<Vec<String>>, values: &Vec<f64>) -> HashMap<String, HashMap<String, f64>> {
        equations.iter().zip(values.iter()).fold(HashMap::new(), |mut acc, (eq, val)| {
            let entry = acc.entry(eq[0].clone()).or_default();
            entry.insert(eq[1].clone(), *val);
            
            let entry = acc.entry(eq[1].clone()).or_default();
            entry.insert(eq[0].clone(), 1.0 / *val);
            
            acc
        })
    }

    fn dfs(graph: &HashMap<String, HashMap<String, f64>>, query: Vec<String>, visited: &mut HashSet<String>, current_sum: Option<f64>) -> Option<f64> {
        visited.insert(query[0].clone());

        let entry = graph.get(&query[0])?;
        if query[0] == query[1] {
            return Some(1.0);
        }
        if let Some(answer) = entry.get(&query[1]) {
            return current_sum.map_or(Some(*answer), |s| Some(s * *answer));
        };

        for path in entry.keys() {
            if !visited.contains(path) {
                if let Some(val) = entry.get(path) {
                    let result = Self::dfs(
                        graph,
                        vec![path.clone(), query[1].clone()],
                        visited,
                        current_sum.map_or(Some(*val), |s| Some(s * *val)),
                    );
                    if result.is_some() {
                        return result;
                    }
                }
            }
        }
        None
    }
}