impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut i = 0;
        let mut j = usize::MAX;
        let mut res = vec![];
        let mut dirs = [matrix[0].len(), matrix.len()-1, matrix[0].len()-1, matrix.len()-2];
        let mut curr_dir = 0;

        loop {
            if (curr_dir == 0 || curr_dir == 2) && (dirs[curr_dir] > matrix[0].len() || dirs[curr_dir] == 0) {
                break;
            } else if (curr_dir == 1 || curr_dir == 3) && (dirs[curr_dir] > matrix.len() || dirs[curr_dir] == 0) {
                break;
            } else {
                for _ in 0..dirs[curr_dir] {
                    if curr_dir == 0 {
                        j += 1;
                    } else if curr_dir == 1 {
                        i += 1;
                    } else if curr_dir == 2 {
                        j -= 1;
                    } else if curr_dir == 3 {
                        i -= 1;
                    }
                    // println!("{i}, {j}");
                    res.push(matrix[i][j]);
                }
                dirs[curr_dir] = dirs[curr_dir] - 2;
                // println!("{:?}", dirs);
                
                if curr_dir == 3 {
                    curr_dir = 0;
                } else {
                    curr_dir += 1;
                }
            }
        }
        res
    }
}