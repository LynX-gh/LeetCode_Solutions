impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }

        let mut currptr = 0;
        let mut dupptr = 1;

        while dupptr < nums.len() {
            if nums[dupptr] != nums[currptr] {
                currptr += 1;
                nums[currptr] = nums[dupptr];
            }
            dupptr += 1;
        }
        (currptr+1) as i32
    }
}