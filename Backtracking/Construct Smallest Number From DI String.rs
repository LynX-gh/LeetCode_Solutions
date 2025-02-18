impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let mut seen = [false; 10];
        let mut res = vec![];

        for i in 1..=9 {
            res.push(i);
            seen[i as usize] = true;
            if Self::backtrack(0, &mut res, &mut seen, &pattern) {
                break;
            }
            res.pop();
            seen[i as usize] = false;
        }
        res.into_iter().map(|i| i.to_string()).collect::<String>()
    }

    fn backtrack(index: usize, res: &mut Vec<i32>, seen: &mut [bool], pattern: &String) -> bool {
        if res.len() == pattern.len() + 1 {
            return true;
        }

        for i in 1..=9 {
            if !seen[i as usize] {
                if pattern.chars().nth(index).unwrap() == 'I' {
                    if i > res[index] {
                        res.push(i);
                        seen[i as usize] = true;
                        if Self::backtrack(index+1, res, seen, pattern) {
                            return true;
                        }
                        res.pop();
                        seen[i as usize] = false;
                    }
                } else {
                    if i < res[index] {
                        res.push(i);
                        seen[i as usize] = true;
                        if Self::backtrack(index+1, res, seen, pattern) {
                            return true;
                        }
                        res.pop();
                        seen[i as usize] = false;
                    }
                }
            }
        }
        false
    }
}