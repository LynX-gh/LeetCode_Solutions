impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut bits = 0;
        let mut temp = num;
        while temp > 0 {
            temp = temp>>1;
            bits += 1;
        }
        num^(2_i32.pow(bits)-1)
    }
}
