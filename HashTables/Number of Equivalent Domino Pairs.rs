use std::collections::HashMap;

// Check both fwd and rev
impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut seen = HashMap::new();

        for i in 0..dominoes.len() {
            let rev = vec![dominoes[i][1], dominoes[i][0]];

            if dominoes[i][0] != dominoes[i][1] {
                res += *seen.get(&dominoes[i]).unwrap_or(&0);
            }
            res += *seen.get(&rev).unwrap_or(&0);

            seen.entry(rev).and_modify(|ctr| *ctr += 1).or_insert(1);
        }
        res
    }
}

// Custom Hash Fxn
impl Solution {
    const ten: i32 = 10;
    fn compute_hash(a: i32, b: i32) -> i32 {
        return (i32::min(a, b)-1)*Self::ten + i32::max(a, b)-1;
    }
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        let n: usize = dominoes.len();
        for i in 0..n {
            let hval = Self::compute_hash(
                dominoes[i][0], dominoes[i][1]
            );
            *cnt.entry(hval).or_insert(0) += 1;
        }
        let mut ans: i32 = 0;
        for v in cnt.values() {
            ans += v*(v-1)/2;
        }
        return ans;
    }
}