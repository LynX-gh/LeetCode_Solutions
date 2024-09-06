impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut num1_ptr = (m-1) as usize;
        let mut num2_ptr = (n-1) as usize;
        let mut zero_ptr = (m+n-1) as usize;

        while num1_ptr < m as usize && num2_ptr < n as usize {
            if nums1[num1_ptr] > nums2[num2_ptr] {
                nums1[zero_ptr] = nums1[num1_ptr];
                num1_ptr -= 1;
            }
            else {
                nums1[zero_ptr] = nums2[num2_ptr];
                num2_ptr -= 1;
            }
            zero_ptr -= 1;
        }
        while num1_ptr < m as usize {
            nums1[zero_ptr] = nums1[num1_ptr];
            num1_ptr -= 1;
            zero_ptr -= 1;
        }
        while num2_ptr < n as usize {
            nums1[zero_ptr] = nums2[num2_ptr];
            num2_ptr -= 1;
            zero_ptr -= 1;
        }
    }
}