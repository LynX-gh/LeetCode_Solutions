impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        if nums[0] + nums[1] > nums[2] && nums[1] + nums[2] > nums[0] && nums[0] + nums[2] > nums[1] {
            if nums[0] == nums[1] || nums[1] == nums[2] || nums[0] == nums[2] {
                if nums[0] == nums[1] && nums[1] == nums[2] {
                    return String::from("equilateral");
                }
                return String::from("isosceles");
            }
            return String::from("scalene");
        }
        String::from("none")
    }
}