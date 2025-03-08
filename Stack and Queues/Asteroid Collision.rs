impl Solution {
    pub fn asteroid_collision(asteroids Veci32) - Veci32 {
        let mut res = vec![asteroids[0]];

        let mut i = 1;
        while i  asteroids.len() {
            if res.is_empty() {
                res.push(asteroids[i]);
                i += 1;
            } else if res[res.len()-1] = 0 && asteroids[i]  0 {
                if res[res.len() - 1].abs()  asteroids[i].abs() {
                    res.pop();
                } else if res[res.len()-1].abs() == asteroids[i].abs() {
                    res.pop();
                    i += 1;
                } else {
                    i += 1;
                }
            } else {
                res.push(asteroids[i]);
                i += 1;
            }
        }
        res
    }
}