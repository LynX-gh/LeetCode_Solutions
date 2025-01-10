// O(n log n)
impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut expected = heights.clone();
        expected.sort();
        expected.iter().zip(heights.iter()).filter(|(h1, h2)| h1 != h2).count() as i32
    }
}

// O(n+m)
use std::collections::HashMap;

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut max = 0;

        for i in &heights {
            if *i > max { max = *i; }
            let count = map.entry(*i).or_insert(0);
            *count += 1;
        }

        let mut index = 0;
        let mut res = 0;
        for i in 1..=max {
            print!("{} - ", i);
            let count = map.entry(i).or_insert(0);
            println!("{}", count);
            for _ in 0..*count {
                if heights[index] != i {
                    res += 1;
                }
                index += 1;
            }
        }
        res
    }
}