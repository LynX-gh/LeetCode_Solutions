use std::collections::BinaryHeap;

impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, kl: i32) -> i64 {
        let k = kl as usize;

        let mut nums_pair: Vec<(i32, i32)> = nums1.into_iter().zip(nums2.into_iter()).collect();
        nums_pair.sort_unstable_by(|p1, p2| p2.1.cmp(&p1.1));

        let mut minheap = BinaryHeap::new();
        let mut n1_sum = 0;
        let mut res = 0;

        for (n1, n2) in nums_pair.into_iter() {
            n1_sum += n1 as i64;
            minheap.push(-n1);

            if minheap.len() > k {
                n1_sum -= minheap.pop().unwrap().abs() as i64;
            }
            if minheap.len() == k {
                res = res.max(n1_sum * n2 as i64);
            }
        }
        res
    }
}
