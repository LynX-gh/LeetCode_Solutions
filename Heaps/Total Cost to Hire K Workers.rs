use std::collections::BinaryHeap;

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let mut res = 0;
        let mut sp = 0;
        let mut ep = costs.len().saturating_sub(1);
        let mut sh = BinaryHeap::new();
        let mut eh = BinaryHeap::new();

        for _ in 0..k {
            while sh.len() < candidates as usize && sp <= ep {
                sh.push((-costs[sp], sp));
                sp += 1;
            }
            while eh.len() < candidates as usize && sp <= ep {
                eh.push((-costs[ep], ep));
                ep = ep.saturating_sub(1);
            }

            match (sh.peek(), eh.peek()) {
                (Some(&(scost, si)), Some(&(ecost, ei))) => {
                    if si != ei {
                        if scost >= ecost {
                            sh.pop();
                            res += scost.abs() as i64;
                        } else {
                            eh.pop();
                            res += ecost.abs() as i64;
                        }
                    } else {
                        // Same index in both heaps (only possible when sp == ep)
                        sh.pop();
                        eh.pop();
                        res += scost.abs() as i64;
                    }
                },
                (Some(&(scost, _)), None) => {
                    sh.pop();
                    res += scost.abs() as i64;
                },
                (None, Some(&(ecost, _))) => {
                    eh.pop();
                    res += ecost.abs() as i64;
                },
                (None, None) => break,
            }
        }
        res
    }
}
