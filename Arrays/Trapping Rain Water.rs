impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut left_max = vec![height[n-1]; n];
        let mut right_max = height[0];
        let mut res = 0;

        for i in (0..n-1).rev() {
            left_max[i] = left_max[i+1].max(height[i]);
        }

        for i in (1..n-1) {
            right_max = height[i-1].max(right_max);
            let water = right_max.min(left_max[i]) - height[i];
            if water > 0 {
                res += water;
            }
        }

        res
    }
}