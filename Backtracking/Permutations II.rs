// O(n! + Sorting Overhead)
use std::collections::HashSet;

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        nums.sort();
        Self::backtrack(0, &mut nums, &mut res);
        res
    }

    fn backtrack(start: usize, nums: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if start == nums.len() {
            res.push(nums.clone());
            return;
        }

        let mut set = HashSet::new();
        for i in start..nums.len() {
            if set.insert(nums[i]) {
                nums.swap(start, i);
                Self::backtrack(start+1, nums, res);
                nums.swap(i, start);
            }
        }
    }
}

// O(n! + n^2)
use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = HashSet::new();
        let mut freq = HashMap::new();
        let mut n = nums.len();

        for &num in nums.iter() {
            freq.entry(num).and_modify(|ctr| *ctr+=1).or_insert(1);
        }

        nums.sort();
        nums.dedup();

        for &num in nums.iter() {
            Self::backtrack(n, &nums, &freq, &mut res, &mut vec![num])
        }

        Vec::from_iter(res)
    }

    fn backtrack(n: usize, nums: &Vec<i32>, freq: &HashMap<i32, i32>, res: &mut HashSet<Vec<i32>>, prev: &mut Vec<i32>) {
        if prev.len() == n {
            res.insert(prev.clone());
        }

        for &val in nums {
            if (prev.iter().filter(|&&x| x == val).count() as i32) < *freq.get(&val).unwrap_or(&0) {
                prev.push(val);
                Self::backtrack(n, nums, freq, res, prev);
                prev.pop();
            }
        }
    }
}