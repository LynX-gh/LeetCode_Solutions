impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, kl: i32) -> i32 {
        let mut res = 0;
        let n = colors.len();
        let k = kl as usize;
        let mut s = 0;

        for i in 1..n+k-1 {
            if colors[i%n] == colors[(i-1)%n] {
                s = i;
            }
            if i - s + 1 > k {
                s += 1;
            }
            if i - s + 1 == k {
                res += 1;
            }
        }
        res
    }
}