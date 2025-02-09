impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut asc_ptr1 = n - 2;

        while asc_ptr1 < n && nums[asc_ptr1] >= nums[asc_ptr1 + 1] {
            asc_ptr1 -= 1;
        }

        if asc_ptr1 < n {
            let mut asc_ptr2 = n-1;
            while nums[asc_ptr1] >= nums[asc_ptr2] {
                asc_ptr2 -= 1;
            }
            nums.swap(asc_ptr1, asc_ptr2);  
        }
        nums[asc_ptr1+1..n].reverse();
    }
}