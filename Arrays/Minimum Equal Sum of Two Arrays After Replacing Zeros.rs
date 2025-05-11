impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let s1 = nums1.iter().fold(0, |acc, &x| if x == 0 { acc + 1 } else { acc + x as i64 } );
        let s2 = nums2.iter().fold(0, |acc, &x| if x == 0 { acc + 1 } else { acc + x as i64 } );

        let z1 = nums1.iter().filter(|&x| *x == 0).count();
        let z2 = nums2.iter().filter(|&x| *x == 0).count();

        if z2 > 0 && s1 >= s2 {
            s1
        } else if z1 > 0 && s2 >= s1 {
            s2
        } else if s1 == s2 {
            s1
        } else {
            -1
        }
    }
}