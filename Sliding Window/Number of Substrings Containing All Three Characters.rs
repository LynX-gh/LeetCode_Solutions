impl Solution {
    pub fn number_of_substrings(s_in: String) -> i32 {
        let s: Vec<char> = s_in.chars().collect();
        let n = s.len();
        let mut res = 0;
        let mut freq = vec![0; 3];
        let mut uniq = 0;
        let mut i = 0;

        for j in 0..n {
            match s[j] {
                'a' => {
                    if freq[0] == 0 {
                        uniq += 1;
                    }
                    freq[0] += 1;
                },
                'b' => {
                    if freq[1] == 0 {
                        uniq += 1;
                    }
                    freq[1] += 1;
                },
                'c' => {
                    if freq[2] == 0 {
                        uniq += 1;
                    }
                    freq[2] += 1;
                },
                _ => (),
            }
            while i <= j && uniq == 3 {
                res += (n - j) as i32;
                match s[i] {
                    'a' => {
                        freq[0] -= 1;
                        if freq[0] == 0 {
                            uniq -= 1;
                        }
                    },
                    'b' => {
                        freq[1] -= 1;
                        if freq[1] == 0 {
                            uniq -= 1;
                        }
                    },
                    'c' => {
                        freq[2] -= 1;
                        if freq[2] == 0 {
                            uniq -= 1;
                        }
                    },
                    _ => (),
                }
                i += 1;
            }
        }
        res
    }
}