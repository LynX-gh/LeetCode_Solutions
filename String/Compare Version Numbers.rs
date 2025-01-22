use std::cmp;
use std::str::FromStr;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut iter1 = version1.split(".").collect::<Vec<&str>>();
        let mut iter2 = version2.split(".").collect::<Vec<&str>>();
        let len1 = iter1.len();
        let len2 = iter2.len();

        for i in 0..cmp::max(len1, len2) {
            let x = i32::from_str(if i < len1 { iter1[i] } else { "0" }).unwrap_or(0);
            let y = i32::from_str(if i < len2 { iter2[i] } else { "0" }).unwrap_or(0);

            if x > y {
                return 1;
            } else if x < y {
                return -1;
            }
        }
        0
    }
}