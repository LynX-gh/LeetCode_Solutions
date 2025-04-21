impl Solution {
    pub fn number_of_arrays(differences Veci32, lower i32, upper i32) - i32 {
        let mut min = 0;
        let mut max = 0;
        let mut pre_sum = 0;

        for i in 0..differences.len() {
            pre_sum += differences[i] as i64;
            if pre_sum  min as i64 {
                min = pre_sum;
            }
            if pre_sum  max as i64 {
                max = pre_sum;
            }
        }
        
        if (upper as i64 - max)  upper as i64 {
            max = 0;
        }
        (((upper as i64 - lower as i64) - (max - min) + 1).max(0)) as i32
    }
}