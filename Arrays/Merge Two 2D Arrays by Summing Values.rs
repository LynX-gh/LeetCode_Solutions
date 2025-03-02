impl Solution {
    pub fn merge_arrays(mut nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ptr1 = 0;
        let mut ptr2 = 0;
        let mut res = Vec::new();

        while ptr1 < nums1.len() && ptr2 < nums2.len() {
            if nums1[ptr1][0] == nums2[ptr2][0] {
                nums1[ptr1][1] += nums2[ptr2][1];
                res.push(nums1[ptr1].clone());
                ptr1 += 1;
                ptr2 += 1;
            } else if nums1[ptr1][0] > nums2[ptr2][0] {
                res.push(nums2[ptr2].clone());
                ptr2 += 1;
            } else {
                res.push(nums1[ptr1].clone());
                ptr1 += 1;
            }
        }

        while ptr2 < nums2.len() {
            res.push(nums2[ptr2].clone());
            ptr2 += 1;
        }

        while ptr1 < nums1.len() {
            res.push(nums1[ptr1].clone());
            ptr1 += 1;
        }

        res
    }
}