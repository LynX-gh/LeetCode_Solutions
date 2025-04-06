impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let doubled = s.clone() + &s;
        let sub = &doubled[1..doubled.len()-1];
        return sub.contains(&s);
    }
}
