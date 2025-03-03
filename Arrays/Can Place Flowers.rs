impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
        let mut res = 0;
        let mut prev = false;

        for i in 0..flowerbed.len()-1 {
            if prev == false && flowerbed[i] == 0 && flowerbed[i+1] == 0 {
                res += 1;
                flowerbed[i] = 1;
            }
            prev = flowerbed[i] == 1;
        }

        if !prev && flowerbed[flowerbed.len()-1] == 0 {
            res += 1;
        }

        res >= n
    }
}