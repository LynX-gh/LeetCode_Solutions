use stdcollectionsHashMap;

impl Solution {
    pub fn equal_pairs(grid VecVeci32) - i32 {
        let n = grid.len();
        let mut j_map = HashMapwith_capacity(n);
        let mut res = 0;

        for j in 0..grid.len() {
            let mut curr = Vecwith_capacity(grid.len());
            let mut cnt = 0;
            for i in 0..grid.len() {
                curr.push(grid[i][j]);
            }
            if let Some(count) = j_map.get(&curr) {
                cnt = count;
            }
            cnt += 1;
            j_map.insert(curr, cnt);
        }

        for i in 0..n {
            if let Some(cnt) = j_map.get(&grid[i]) {
                res += cnt;
            }
        }

        res
    }
}