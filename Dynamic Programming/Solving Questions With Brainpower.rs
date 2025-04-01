impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let mut res = vec![0; questions.len()];
        res[questions.len() - 1] = questions[questions.len()-1][0] as i64;

        for i in (0..questions.len()-1).rev() {
            let mut cur_pts = questions[i][0] as i64;
            if (i + 1 + questions[i][1] as usize) < questions.len() {
                cur_pts += res[questions[i][1] as usize + i + 1];
            }

            res[i] = res[i+1].max(cur_pts);
        }
        res[0]
    }
}
