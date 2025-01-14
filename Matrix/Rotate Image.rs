impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let mut temp;
        let n = matrix.len();
        for i in 0..n {
            for j in 0..i {
                temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }
        for i in 0..n {
            matrix[i].reverse();
        }
    }
}