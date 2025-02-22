impl Solution {
    pub fn longest_palindrome(s String) - String {
         if s == s.chars().rev().collectString() {
             return s;
         }

        let mut ptr1 = 0;
        let mut ptr2 = 0;
        let mut res1 = 0;
        let mut res2 = 0;
        let sb = s.as_bytes();

        for i in 0..sb.len()-1 {
            ptr1 = i;
            ptr2 = i;
            while ptr1-1  s.len() && ptr2+1  s.len() && sb[ptr1-1] == sb[ptr2+1] {
                ptr1 -= 1;
                ptr2 += 1;
            }
            if (res2 - res1)  (ptr2 - ptr1) {
                res1 = ptr1;
                res2 = ptr2;
            }

            ptr1 = i;
            ptr2 = i;
            if sb[ptr1] == sb[ptr2+1] {
                ptr1 = i;
                ptr2 = i+1;
                while ptr1-1  s.len() && ptr2+1  s.len() && sb[ptr1-1] == sb[ptr2+1] {
                    ptr1 -= 1;
                    ptr2 += 1;
                }
                if (res2 - res1)  (ptr2 - ptr1) {
                    res1 = ptr1;
                    res2 = ptr2;
                }
            }
        }

        s[res1..=res2].to_string()
    }
}