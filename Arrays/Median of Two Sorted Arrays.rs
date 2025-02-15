impl Solution {
    pub fn find_median_sorted_arrays(nums1 Veci32, nums2 Veci32) - f64 {
        let total = nums1.len() + nums2.len();
        let mut curr = 0;
        let mut prev = 0;
        let mut ptr1 = 0;
        let mut ptr2 = 0;

        for _ in 0..=total2 {
            prev = curr;

            if ptr1  nums1.len() && ptr2  nums2.len() {
                if nums1[ptr1]  nums2[ptr2] {
                    curr = nums2[ptr2];
                    ptr2 += 1;
                } else {
                    curr = nums1[ptr1];
                    ptr1 += 1;
                }
            } else if ptr1  nums1.len() {
                curr = nums1[ptr1];
                ptr1 += 1;
            } else {
                curr = nums2[ptr2];
                ptr2 += 1;
            }
        }

        if total % 2 == 1 {
            curr as f64
        } else {
            (prev as f64 + curr as f64)  2.0
        }
    }
}