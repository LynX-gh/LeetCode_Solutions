use std::collections::HashMap;

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut grp_map = HashMap::new();
        let mut res = Vec::new();
        for i in 0..group_sizes.len() {
            let mut grp = grp_map.entry(group_sizes[i]).and_modify(|arr: &mut Vec<i32>| arr.push(i as i32)).or_insert(vec![i as i32]);
            if grp.len() == group_sizes[i] as usize {
                res.push(grp.clone());
                grp.clear();
            }
        }
        res
    }
}
