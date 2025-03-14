impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut left = 0;
        let mut right = queries.len();

        if !zero_array(&nums, &queries, right) {
            return -1;
        }

        while left <= right && right <= queries.len() {
            println!("{left} - {right}");
            let mid = left + (right - left) / 2;
            if zero_array(&nums, &queries, mid) {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        left as i32
    }
}

fn zero_array(nums: &Vec<i32>, queries: &Vec<Vec<i32>>, k: usize) -> bool {
    let n = nums.len();
    let mut diff = vec![0; n+1];

    for i in 0..k {
        let start = queries[i][0] as usize;
        let end = queries[i][1] as usize;
        let val = queries[i][2];

        diff[start] += val;
        diff[end+1] -= val; 
    }

    let mut sum = 0;
    for i in 0..n {
        sum += diff[i];
        if sum < nums[i] {
            return false;
        }
    }
    true
}