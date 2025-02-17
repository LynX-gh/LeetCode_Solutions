use std::collections::HashSet;

impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut map = [0; 26];
        let mut unique_chars = HashSet::new();
        for ch in tiles.chars() {
            map[ch as usize - 'A' as usize] += 1;
            unique_chars.insert(ch as usize - 'A' as usize);
        }
        
        let mut res = 0;
        Self::backtrack(&mut map, &mut res, &unique_chars);
        res
    }
    
    fn backtrack(map: &mut [i32; 26], res: &mut i32, unique_chars: &HashSet<usize>) {
        for &i in unique_chars {
            if map[i] > 0 {
                map[i] -= 1;
                *res += 1;
                Self::backtrack(map, res, unique_chars);
                map[i] += 1;
            }
        }
    }
}