use std::collections::HashMap;

impl Solution {
    pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut color_count = HashMap::new();
        let mut color_map = HashMap::new();
        let mut res = vec![];

        for query in queries {
            let i = query[0] as usize;
            let j = query[1] as usize;

            let color_ctr = color_map.entry(i).or_insert(0);
            if *color_ctr != 0 { 
                let ctr = color_count.entry(*color_ctr).or_insert(0);
                if *ctr == 1 {
                    color_count.remove(color_ctr);
                } else {
                    *ctr -= 1;
                }
            }
            color_count.entry(j).and_modify(|ctr| *ctr += 1).or_insert(1);
            res.push(color_count.keys().len() as i32);
            color_map.insert(i, j);
        }
        res
    }
}