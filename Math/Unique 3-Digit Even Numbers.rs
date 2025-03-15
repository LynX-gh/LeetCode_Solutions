impl Solution {
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        let mut map = vec![0; 10];
        for d in digits {
            map[d as usize] += 1;
        }

        let mut res = 0;

        for i in (100..1000).step_by(2) {
            let mut temp = vec![0; 10];
            let mut num = i;

            for _ in 0..3 {
                temp[(num%10) as usize] += 1;
                num /= 10;
            }
            if (0..10).all(|x| temp[x] <= map[x]) {
                res += 1;
            }
        }
        res
    }
}