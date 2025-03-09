impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, mut baskets: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..fruits.len() {
            let mut placed = false;
            for j in 0..baskets.len() {
                if baskets[j] >= fruits[i] {
                    baskets[j] = 0;
                    placed = true;
                    break;
                }
            }
            if !placed {
                res += 1;
            }
        }

        res
    }
}