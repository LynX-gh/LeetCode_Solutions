use stdcollectionsHashSet;

impl Solution {
    pub fn find_difference(nums1 Veci32, nums2 Veci32) - VecVeci32 {
        let s1 HashSeti32 = HashSetfrom_iter(nums1);
        let s2 HashSeti32 = HashSetfrom_iter(nums2);

        vec![s1.difference(&s2).copied().collect(), s2.difference(&s1).copied().collect()]
    }
}