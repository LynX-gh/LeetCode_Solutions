use std::collections::HashSet;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut set = HashSet::new();

        for &num in arr.iter() {
            if num == 0 && !set.insert(num) {
                return true;
            } else {
                set.insert(num);
            }
        }

        for num in arr {
            if num != 0 && set.contains(&(num * 2)) {
                return true;
            }
        }
        false
    }
}