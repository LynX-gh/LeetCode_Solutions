impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut min = 1;
        let mut max = *piles.iter().max().unwrap();
        let mut res = 0;

        while min <= max {
            let mid = min + (max - min) / 2;
            let hrs = piles.iter().fold(0, |acc, &x| acc + (x + mid - 1)/mid);

            if hrs < 1 {
                break;
            }

            if hrs <= h {
                res = mid;
                max = mid - 1;
            } else {
                min = mid + 1;
            }
        }
        res
    }
}