use std::collections::VecDeque;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut rad = VecDeque::new();
        let mut dir = VecDeque::new();
        let mut n = senate.len();

        for (idx, ch) in senate.chars().enumerate() {
            if ch == 'R' {
                rad.push_back(idx);
            } else {
                dir.push_back(idx);
            }
        }

        while let (Some(rd), Some(dr)) = (rad.front(), dir.front()) {
            if rd < dr {
                rad.push_back(n);
                n += 1;
            } else {
                dir.push_back(n);
                n += 1;
            }
            rad.pop_front();
            dir.pop_front();
        }
        if rad.is_empty() {
            String::from("Dire")
        } else {
            String::from("Radiant")
        }
    }
}