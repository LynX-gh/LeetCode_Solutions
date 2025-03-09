impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut res = f64::MIN;
        let mut window = 0.0;
        let klen = k as usize;
        
        for i in 0..klen {
            window += nums[i] as f64;
        }

        res = window;
        for i in 0..nums.len() - klen {
            window -= nums[i] as f64;
            window += nums[i + klen] as f64;
            res = res.max(window);
        }

        res / k as f64
    }
}