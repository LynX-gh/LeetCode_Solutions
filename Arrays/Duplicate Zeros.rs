impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut orig_len = arr.len();
        let mut i = 0;
        while i < orig_len {
            if arr[i] == 0 {
                arr.insert(i, 0);
                i += 1;
            }
            i += 1;
        }
        arr.truncate(orig_len)
    }
}

impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut orig_len = arr.len();
        let mut i = 0;
        while i < orig_len {
            if arr[i] == 0 {
                arr.pop();
                arr.insert(i, 0);
                i += 1;
            }
            i += 1;
        }
        // arr.truncate(orig_len)
    }
}