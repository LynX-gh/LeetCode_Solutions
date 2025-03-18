impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        potions.sort();
        let mut res = Vec::with_capacity(spells.len());

        for i in 0..spells.len() {
            let mut min = (success + spells[i] as i64 - 1)/spells[i] as i64;
            res.push(potions.len() as i32 - Self::search(&potions, min));
        }
        res
    }

    fn search(potions: &Vec<i32>, n: i64) -> i32 {
        let mut l = 0;
        let mut r = potions.len();

        while l < r {
            let mid = l + (r - l) / 2;
            if potions[mid] as i64 >= n {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        r as i32
    }
}