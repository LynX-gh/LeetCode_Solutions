impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut map = vec![0; nums.len()+1];
        let mut res = vec![];

        for i in nums.into_iter() {
            map[i as usize] += 1;
        }

        for i in 1..map.len() {
            if map[i] == 0 {
                res.push(i as i32);
            }
        }
        res
    }
}