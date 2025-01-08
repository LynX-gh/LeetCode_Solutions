use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return vec![];
        }

        let map = HashMap::from([
            ('2', vec!["a", "b", "c"]),
            ('3', vec!["d", "e", "f"]),
            ('4', vec!["g", "h", "i"]),
            ('5', vec!["j", "k", "l"]),
            ('6', vec!["m", "n", "o"]),
            ('7', vec!["p", "q", "r", "s"]),
            ('8', vec!["t", "u", "v"]),
            ('9', vec!["w", "x", "y", "z"])
        ]);

        let mut result = vec![String::new()];

        for digit in digits.chars() {
            if let Some(letters) = map.get(&digit) {
                let mut temp = vec![];
                for combination in &result {
                    for &letter in letters {
                        let mut new_combination = combination.clone();
                        new_combination.push_str(letter);
                        temp.push(new_combination);
                    }
                }
                result = temp;
            }
        }

        result
    }
}