// BinaryHeap

use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut min_heap = nums.iter().map(|&x| Reverse(x as i64)).collect::<BinaryHeap<_>>();

        let mut operations = 0;
        while let Some(Reverse(x)) = min_heap.pop() {
            if x >= k as i64 {
                break;
            }
            if let Some(Reverse(y)) = min_heap.pop() {
                min_heap.push(Reverse(x * 2 + y)); // expression simplified due to min heap property
            }
            operations += 1;
        }

        operations
    }
}


// BTreeMap

use std::collections::BTreeMap;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, mut k: i32) -> i32 {
        let k = k as i64;
        let mut map = BTreeMap::<i64, i32>::new();
        let mut res = 0;

        for num in nums.into_iter() {
            map.entry(num as i64).and_modify(|ctr| *ctr += 1).or_insert(1);
        }

        while let Some((key1, cnt1)) = map.pop_first() {
            if key1 >= k {
                break;
            }

            res += 1;
            let mut new_num = -1;
            if cnt1 > 1 {
                if cnt1-2 != 0 {
                    map.insert(key1, cnt1-2);
                }
                new_num = key1 * 2 + key1;
            } else if let Some((key2, cnt2)) = map.pop_first() {
                if cnt2 > 1 {
                    map.insert(key2, cnt2-1);
                }
                new_num = key1.min(key2) * 2 + key1.max(key2);
            }
            map.entry(new_num).and_modify(|ctr| *ctr += 1).or_insert(1);
        }

        res
    }
}