impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        (1..=n).filter(|&i| Self::can_partition(i * i, i)).map(|i| i * i).sum()
    }

    fn can_partition(square: i32, target: i32) -> bool {
        let mut digits = Vec::new();
        let mut num = square;
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        digits.reverse();
        Self::backtrack(&digits, 0, 0, target)
    }

    fn backtrack(digits: &[i32], index: usize, current_sum: i32, target: i32) -> bool {
        if index == digits.len() {
            return current_sum == target;
        }
        let mut num = 0;
        for i in index..digits.len() {
            num = num * 10 + digits[i];
            if num > target {
                break;
            }
            if Self::backtrack(digits, i + 1, current_sum + num, target) {
                return true;
            }
        }
        false
    }
}