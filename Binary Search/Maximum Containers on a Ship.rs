// O(n)
impl Solution {
    pub fn max_containers(n: i32, w: i32, max_weight: i32) -> i32 {
        (n*n).min(max_weight / w)
    }
}

// O(log n)
impl Solution {
    pub fn max_containers(n: i32, w: i32, max_weight: i32) -> i32 {
        let mut min = 1;
        let mut max = n * n;
        let mut res = 0;

        while min <= max {
            let mid = min + (max - min)/2;
            if (mid * w) <= max_weight {
                res = mid;
                min = mid + 1;
            } else {
                max = mid - 1;
            }
        }
        res
    }
}