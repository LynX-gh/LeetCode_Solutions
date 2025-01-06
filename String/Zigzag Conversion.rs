impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let mut res = String::new();
        let bytes = s.as_bytes();
        let increment = 2 * (num_rows - 1) as usize;

        for row in 0..num_rows as usize {
            let mut i = row;
            while i < bytes.len() {
                res.push(bytes[i] as char);
                if row > 0 && row < (num_rows as usize - 1) {
                    let diagonal_index = i + increment - 2 * row;
                    if diagonal_index < bytes.len() {
                        res.push(bytes[diagonal_index] as char);
                    }
                }
                i += increment;
            }
        }
        res
    }
}