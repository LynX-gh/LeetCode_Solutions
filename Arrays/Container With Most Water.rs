impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut start_ptr = 0;
        let mut end_ptr = height.len()-1;
        let mut max_cap = 0;

        while start_ptr < end_ptr {
            if height[start_ptr] < height[end_ptr] {
                max_cap = i32::max(max_cap, (height[start_ptr] * (end_ptr - start_ptr) as i32));
                start_ptr += 1;
            }
            else {
                max_cap = i32::max(max_cap, (height[end_ptr] * (end_ptr - start_ptr) as i32));
                end_ptr -= 1;
            }
        }
        max_cap
    }
}