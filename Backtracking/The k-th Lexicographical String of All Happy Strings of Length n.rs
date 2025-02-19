impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let mut res = vec![];
        let mut chars = ['a', 'b', 'c'];

        for ch in chars {
            if Self::backtrack(&mut ch.to_string(), &mut res, &chars, n as usize, k as usize) {
                return res[k as usize - 1].clone();
            }
        }
        String::new()
    }

    fn backtrack(curr: &mut String, res: &mut Vec<String>, chars: &[char], target: usize, max_cap: usize) -> bool {
        if curr.len() == target {
            res.push(curr.clone());
            return res.len() == max_cap;
        }

        for &ch in chars {
            if curr.chars().nth(curr.len()-1).unwrap() != ch {
                curr.push(ch);
                if Self::backtrack(curr, res, chars, target, max_cap) {
                    return true;
                }
                curr.pop();
            }
        }
        false
    }
}