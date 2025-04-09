impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut res = vec![area,1];
        let mut min_dif = area - 1;
        
        for i in 1..area.isqrt()+1 {
            if area % i == 0 && (area / i) - i < min_dif && area/i >= i {
                min_dif = (area/i) - i;
                res[0] = area/i;
                res[1] = i;
            }
        }
        res
    }
}
