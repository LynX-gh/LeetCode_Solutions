impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {return false;}

        if x < 10 {return true;}

        let num_str = x.to_string();
        let byte_str = num_str.as_bytes();

        let mut start_ref = 0;
        let mut end_ref = num_str.len()-1;

        while start_ref <= end_ref {
            if byte_str[start_ref] != byte_str[end_ref] {
                return false;
            }
            start_ref += 1;
            end_ref -= 1;
        }
        true
    }
}