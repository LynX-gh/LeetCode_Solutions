impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let n = digits.len();
        let mut carry = false;

        if digits[n-1] == 9 {
            digits[n-1] = 0;
            carry = true;
        } else {
            digits[n-1] += 1;
        }

        for i in (0..n-1).rev() {
            if !carry {
                break;
            }

            if digits[i] == 9 {
                digits[i] = 0;
            } else {
                digits[i] += 1;
                carry = false;
            }
        }

        if carry {
            digits.insert(0, 1);
        }
        digits
    }
}