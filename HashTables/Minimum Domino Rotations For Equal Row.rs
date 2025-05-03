use std::collections::HashMap;

impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let mut freq_map = vec![0; 7];

        for i in 0..tops.len() {
            freq_map[tops[i] as usize] += 1;
            freq_map[bottoms[i] as usize] += 1;
        }

        let mut max = 0;
        for i in 0..7 {
            if freq_map[i] >= tops.len() {
                max = i as i32;
            }
        }

        if max == 0 {
            return -1;
        }

        let mut min_t = 0;
        let mut min_b = 0;

        for i in 0..tops.len() {
            if tops[i] != max {
                if bottoms[i] != max {
                    return -1;
                }
                min_t += 1;
            }
            if bottoms[i] != max {
                if tops[i] != max {
                    return -1;
                }
                min_b += 1;
            }
        }
        
        min_t.min(min_b)
    }
}