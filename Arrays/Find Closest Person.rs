impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        let d1 = (x - z).abs();
        let d2 = (y - z).abs();

        if d1 < d2 {
            return 1;
        } 
        if d2 < d1 {
            return 2;
        }
        0
    }
}
