impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut res = vec![-1; nums.len()];
        let mut low = 0;
        let mut same = 0;
        let mut high = 0;

        for &num in nums.iter() {
            if num < pivot {
                low += 1;
            } else if num == pivot {
                same += 1;
            }
        }

        high = low + same;
        same = low;
        low = 0;
        for &num in nums.iter() {
            if num < pivot {
                res[low] = num;
                low += 1;
            } else if num == pivot {
                res[same] = num;
                same += 1;
            } else {
                res[high] = num;
                high += 1;
            }
        }

        res
    }
}