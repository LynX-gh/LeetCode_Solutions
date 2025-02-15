use std::collections::{HashMap, BinaryHeap};

impl Solution {
    pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut cap_prof = capital
            .into_iter()
            .zip(profits.into_iter())
            .collect::<Vec<_>>();
        cap_prof.sort_unstable_by_key(|&(cap, _)| cap);

        let mut available = BinaryHeap::new();
        let mut unavailable = &cap_prof[..];

        for _ in 0..k {
            let slice = unavailable.partition_point(|&(cap, _)| cap <= w);
            let working_slice = &unavailable[..slice];
            unavailable = &unavailable[slice..];

            for &(_, prof) in working_slice {
                available.push(prof);
            }

            if let Some(prof) = available.pop() {
                w += prof;
            } else {
                break;
            }
        }
        w
    }
}