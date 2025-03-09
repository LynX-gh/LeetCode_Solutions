use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_max_sum(nums1: Vec<i32>, nums2: Vec<i32>, klen: i32) -> Vec<i64> {
        let n = nums1.len();
        let k = klen as usize;
        let mut res = vec![0; nums1.len()];

        let mut value_indices: Vec<(i32, usize)> = nums1.iter().cloned().zip(0..n).collect();
        value_indices.sort_unstable();

        let mut heap = BinaryHeap::new();
        let mut sum_k: i64 = 0;
        let mut j = 0;

        for i in 0..n {
            let (val_i, original_idx) = value_indices[i];

            while j < i {
                let (val_j, idx_j) = value_indices[j];

                if val_j >= val_i {
                    break;
                }

                heap.push(Reverse(nums2[idx_j] as i64));
                sum_k += nums2[idx_j] as i64;

                if heap.len() > k {
                    if let Some(Reverse(min_val)) = heap.pop() {
                        sum_k -= min_val;
                    }
                }
                
                j += 1;
            }

            res[original_idx] = sum_k;
        }

        res
    }
}