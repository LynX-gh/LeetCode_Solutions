use std::collections::HashMap;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut res = 0;
        for i in 0..answers.len() {
            let mut ctr = map.entry(answers[i]).and_modify(|ctr| *ctr += 1).or_insert(1);
            if *ctr > answers[i] + 1 {
                res += answers[i] + 1;
                *ctr = 1;
            }
        }

        for (key, val) in map.iter() {
            if *val > 0 {
                res += *key + 1;
            }
        }
        res
    }
}
