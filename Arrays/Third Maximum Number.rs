impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut expected = nums.clone();
        expected.sort();

        let mut uniq = 1;
        let mut last_uniq = expected[expected.len()-1];
        for i in expected.iter().rev() {
            if *i != last_uniq {
                uniq += 1;
                last_uniq = *i;
            }
            if uniq == 3 {
                return last_uniq;
            }
        }
        expected[expected.len()-1]
    }
}