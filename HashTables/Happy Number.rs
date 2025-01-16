// O(log n)
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut fast_sum = Self::next_happy(Self::next_happy(n));
        let mut slow_sum = Self::next_happy(n);

        while fast_sum != slow_sum {
            slow_sum = Self::next_happy(slow_sum);
            fast_sum = Self::next_happy(Self::next_happy(fast_sum));
            if fast_sum == 1 || slow_sum == 1 {
                return true;
            }
            if fast_sum < 10 || slow_sum < 10 {
                return false;
            }
        }
        slow_sum == 1
    }

    fn next_happy(mut n: i32) -> i32 {
        let mut sum = 0;
        while n >= 1 {
            sum += (n % 10)*(n % 10);
            n /= 10;
        }
        sum
    }
}

// O(k)
use std::collections::HashSet;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut seen = HashSet::new();
        let mut sum = n;

        while sum != 1 {
            sum = Self::next_happy(sum);
            if sum < 10 && seen.contains(&sum) {
                return false;
            }
            seen.insert(sum);
        }
        true
    }

    fn next_happy(mut n: i32) -> i32 {
        let mut sum = 0;
        while n >= 1 {
            sum += (n % 10)*(n % 10);
            n /= 10;
        }
        sum
    }
}