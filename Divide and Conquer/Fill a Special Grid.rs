impl Solution {
    pub fn special_grid(n: i32) -> Vec<Vec<i32>> {
        let size = 1 << n;
        let mut res = vec![vec![0; size]; size];
        Self::fill(&mut res, n, 0, 0, 0);
        res
    }

    fn fill(res: &mut Vec<Vec<i32>>, n: i32, r: usize, c: usize, base: usize) {
        if n == 0 {
            res[r][c] = base as i32;
            return;
        }

        let size = 1 << n;
        let half = size >> 1;
        let block = (size * size) >> 2;

        Self::fill(res, n-1, r, c + half, base + 0 * block);
        Self::fill(res, n-1, r + half, c + half, base + 1 * block);
        Self::fill(res, n-1, r + half, c, base + 2 * block);
        Self::fill(res, n-1, r, c, base + 3 * block);
    }
}