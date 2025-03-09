impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let block_ch: Vec<char> = blocks.chars().collect();
        let mut s = 0;
        let mut res = k;
        let mut curr = 0;

        for i in 0..blocks.len() {
            if block_ch[i] == 'W' {
                curr += 1;
            }
            if i - s + 1 > k as usize {
                if block_ch[s] == 'W' {
                    curr -= 1;
                }
                s += 1;
            }
            if i - s + 1 == k as usize {
                res = res.min(curr);
            }
        }
        res
    }
}