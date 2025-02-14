impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res = vec![-1, -1];
        
        if nums.len() == 0 {
            return res;
        }

        let mut left = 0;
        let mut right = nums.len()-1;
        let mut mid = nums.len();

        while left <= right && right < nums.len() {
            println!("{mid}");
            mid = (left + right) / 2;
            if nums[mid] == target {
                res[0] = mid as i32;
                res[1] = mid as i32;
                break;
            } else if nums[mid] > target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        while mid > 0 && nums[mid-1] == target {
            mid -= 1;
            res[0] = mid as i32;
        }
        
        while mid < nums.len() - 1 && nums[mid+1] == target {
            mid += 1;
            res[1] = mid as i32;
        }

        res
    }
}