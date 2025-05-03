use std::collections::BTreeMap;

impl Solution {
    pub fn max_task_assign(tasks: Vec<i32>, workers: Vec<i32>, pills: i32, strength: i32) -> i32 {
        let mut tasks = tasks;
        let mut workers = workers;
        tasks.sort();
        workers.sort();
        let n = tasks.len();
        let m = workers.len();
        let (mut left, mut right, mut ans) = (1, m.min(n), 0);

        while left <= right {
            let mid = (left + right) / 2;
            if Self::check(&tasks, &workers, pills, strength, mid) {
                ans = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        ans as i32
    }

    fn check(tasks: &[i32], workers: &[i32], pills: i32, strength: i32, mid: usize) -> bool {
        let mut p = pills;
        let mut ws = BTreeMap::new();
        for &w in workers.iter().skip(workers.len() - mid) {
            *ws.entry(w).or_insert(0) += 1;
        }
        for &t in tasks.iter().take(mid).rev() {
            if let Some((&max_key, _)) = ws.iter().next_back() {
                if max_key >= t {
                    *ws.get_mut(&max_key).unwrap() -= 1;
                    if ws[&max_key] == 0 {
                        ws.remove(&max_key);
                    }
                } else {
                    if p == 0 {
                        return false;
                    }
                    if let Some((&key, _)) = ws.range(t - strength..).next() {
                        *ws.get_mut(&key).unwrap() -= 1;
                        if ws[&key] == 0 {
                            ws.remove(&key);
                        }
                        p -= 1;
                    } else {
                        return false;
                    }
                }
            }
        }
        true
    }
}