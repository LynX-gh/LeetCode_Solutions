// O(n)
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        if numbers.len() < 3 {
            return vec![1, 2];
        }

        let mut lptr = 0;
        let mut rptr = numbers.len()-1;

        while lptr < rptr {
            match (numbers[lptr] + numbers[rptr]).cmp(&target) {
                std::cmp::Ordering::Equal => return vec![lptr as i32 + 1, rptr as i32 + 1],
                std::cmp::Ordering::Greater => rptr -= 1,
                std::cmp::Ordering::Less => lptr += 1
            }
        }
        vec![0, 0]
    }
}

// O(2n)
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        if numbers.len() < 3 {
            return vec![1,2];
        }

        let mut map = HashMap::new();

        for i in 0..numbers.len() {
            map.insert(numbers[i], i);
        }

        for i in 0..numbers.len() {
            match map.get(&(target - numbers[i])) {
                Some(&ind) => return vec![i as i32 + 1, ind as i32 + 1],
                _ => continue
            }
        }
        
        vec![0, 0]
    }
}

// O(n log m)
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        if numbers.len() < 3 {
            return vec![1,2];
        }
        for i in 0..numbers.len() {
            let res = Self::binary_search(&numbers, i+1, target-numbers[i]);
            if res < numbers.len() {
                return vec![i as i32 + 1, res as i32 + 1];
            }
        }
        vec![0, 0]
    }

    pub fn binary_search(nums: &[i32], i: usize, target: i32) -> usize {
        let mut start = i;
        let mut end = nums.len() - 1;
        let mut mid = (start + end) / 2;

        while start <= end && start < nums.len() && end < nums.len(){
            match target.cmp(&nums[mid]) {
                std::cmp::Ordering::Equal => return mid,
                std::cmp::Ordering::Greater => {
                    start = mid+1;
                    mid = (start + end) / 2;
                },
                std::cmp::Ordering::Less => {
                    end = mid-1;
                    mid = (start + end) / 2;
                }
            }
        }
        nums.len() + 1
    }
}