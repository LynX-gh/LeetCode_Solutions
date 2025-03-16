impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut i = 0;
        let mut j = height.len()-1;

        while i < j {
            res = res.max((j - i) as i32 * height[i].min(height[j]));
            if height[i] < height[j] {
                i += 1;                
            } else {
                j -= 1;
            }
        }
        res
    }
}