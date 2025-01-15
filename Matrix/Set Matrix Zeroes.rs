use std::collections::HashSet;
use std::cmp::max;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut set = HashSet::new();

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 0 {
                    set.insert(i);
                    set.insert(300+j);
                }
            }
        }

        for val in &set {
            if *val >= 300 {
                for i in 0..matrix.len() {
                    matrix[i][*val-300] = 0;
                }
            }
            else {
                for j in 0..matrix[0].len() {
                    matrix[*val][j] = 0;
                }
            }
        }
    }
}