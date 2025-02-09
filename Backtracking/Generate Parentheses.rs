impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut count_map = [1, 0];
        let mut opening_string = String::from("(");
        let mut res = vec![];

        Self::backtrack(&mut opening_string, &mut res, &mut count_map, n);
        res
    }

    fn backtrack(prev: &mut String, res: &mut Vec<String>, count: &mut [i32], n: i32) {
        if count[1] == n {
            res.push(prev.clone());
            return;
        }

        if count[0] > count[1] && count[1] < n {
            count[1] += 1;
            prev.push(')');
            Self::backtrack(prev, res, count, n);
            count[1] -= 1;
            prev.pop();
        }
        if count[0] < n {
            count[0] += 1;
            prev.push('(');
            Self::backtrack(prev, res, count, n);
            count[0] -= 1;
            prev.pop();
        }
    }
}