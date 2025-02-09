use stdcollectionsHashMap;

impl Solution {
    pub fn count_bad_pairs(nums Veci32) - i64 {
        let mut bad_pairs = 0_i64;
        let mut map = HashMapnew();

        for i in 0..nums.len() {
            let ctr = map.entry(i as i32 - nums[i]).or_insert(0);
            bad_pairs += i as i64 - ctr;
            ctr += 1;
        }

        bad_pairs
    }
}