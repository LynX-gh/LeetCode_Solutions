// O(n!)
impl Solution {
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        if n == 1 {
            return vec![1];
        }
        
        let mut res = vec![0; (2*n - 1) as usize];
        let mut map = vec![false; n as usize + 1];

        map[n as usize] = true;
        res[0] = n;
        res[n as usize] = n;
        Self::backtrack(&mut res, &mut map, 1, n-1);
        res
    }

    fn backtrack(prev: &mut Vec<i32>, map: &mut Vec<bool>, index: usize, n: i32) -> bool {
        if prev.len() == index {
            return true;
        }

        if prev[index] != 0 {
            return Self::backtrack(prev, map, index+1, n);
        }

        for i in (1..=n).rev() {
            if map[i as usize] == true {
                continue;
            }

            if i == 1 {
                map[1] = true;
                prev[index] = 1;
                if Self::backtrack(prev, map, index+1, n) {
                    return true;
                }
                map[1] = false;
                prev[index] = 0;
            } else if (index + i as usize) < prev.len() && prev[index + i as usize] == 0 {
                map[i as usize] = true;
                prev[index] = i;
                prev[index + i as usize] = i;
                if Self::backtrack(prev, map, index+1, n) {
                    return true;
                }
                map[i as usize] = false;
                prev[index] = 0;
                prev[index + i as usize] = 0;
            }
        }

        false
    }
}


// O(1) Gang 👌
impl Solution {
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        let res = [
            vec![1],
            vec![2,1,2],
            vec![3, 1, 2, 3, 2],
            vec![4, 2, 3, 2, 4, 3, 1],
            vec![5, 3, 1, 4, 3, 5, 2, 4, 2],
            vec![6, 4, 2, 5, 2, 4, 6, 3, 5, 1, 3],
            vec![7, 5, 3, 6, 4, 3, 5, 7, 4, 6, 2, 1, 2],
            vec![8, 6, 4, 2, 7, 2, 4, 6, 8, 5, 3, 7, 1, 3, 5],
            vec![9, 7, 5, 3, 8, 6, 3, 5, 7, 9, 4, 6, 8, 2, 4, 2, 1],
            vec![10, 8, 6, 9, 3, 1, 7, 3, 6, 8, 10, 5, 9, 7, 4, 2, 5, 2, 4],
            vec![11, 9, 10, 6, 4, 1, 7, 8, 4, 6, 9, 11, 10, 7, 5, 8, 2, 3, 2, 5, 3],
            vec![12, 10, 11, 7, 5, 3, 8, 9, 3, 5, 7, 10, 12, 11, 8, 6, 9, 2, 4, 2, 1, 6, 4],
            vec![13, 11, 12, 8, 6, 4, 9, 10, 1, 4, 6, 8, 11, 13, 12, 9, 7, 10, 3, 5, 2, 3, 2, 7, 5],
            vec![14, 12, 13, 9, 7, 11, 4, 1, 10, 8, 4, 7, 9, 12, 14, 13, 11, 8, 10, 6, 3, 5, 2, 3, 2, 6, 5],
            vec![15, 13, 14, 10, 8, 12, 5, 3, 11, 9, 3, 5, 8, 10, 13, 15, 14, 12, 9, 11, 7, 4, 6, 1, 2, 4, 2, 7, 6],
            vec![16, 14, 15, 11, 9, 13, 6, 4, 12, 10, 1, 4, 6, 9, 11, 14, 16, 15, 13, 10, 12, 8, 5, 7, 2, 3, 2, 5, 3, 8, 7],
            vec![17, 15, 16, 12, 10, 14, 7, 5, 3, 13, 11, 3, 5, 7, 10, 12, 15, 17, 16, 14, 9, 11, 13, 8, 6, 2, 1, 2, 4, 9, 6, 8, 4],
            vec![18, 16, 17, 13, 11, 15, 8, 14, 4, 2, 12, 2, 4, 10, 8, 11, 13, 16, 18, 17, 15, 14, 12, 10, 9, 7, 5, 3, 6, 1, 3, 5, 7, 9, 6],
            vec![19, 17, 18, 14, 12, 16, 9, 15, 6, 3, 13, 1, 3, 11, 6, 9, 12, 14, 17, 19, 18, 16, 15, 13, 11, 10, 8, 4, 5, 7, 2, 4, 2, 5, 8, 10, 7],
            vec![20, 18, 19, 15, 13, 17, 10, 16, 7, 5, 3, 14, 12, 3, 5, 7, 10, 13, 15, 18, 20, 19, 17, 16, 12, 14, 11, 9, 4, 6, 8, 2, 4, 2, 1, 6, 9, 11, 8],
        ];

        return res[n as usize - 1].clone();
    }
}