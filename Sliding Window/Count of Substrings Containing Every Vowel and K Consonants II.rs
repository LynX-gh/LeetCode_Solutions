use std::collections::HashMap;

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        let mut vow = HashMap::with_capacity(5);
        let mut word_ch: Vec<char> = word.chars().collect();
        let n = word_ch.len();
        let mut next_con = vec![n; n];
        let mut con = 0;
        let mut res = 0;
        let mut s = 0;

        let mut last_consonant = n;
        for i in (0..n).rev() {
            next_con[i] = last_consonant;
            if !(word_ch[i] == 'a' || word_ch[i] == 'e' || word_ch[i] == 'i' || word_ch[i] == 'o' || word_ch[i] == 'u') {
                last_consonant = i;
            }
        }

        for i in 0..n {
            match word_ch[i] {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    vow.entry(word_ch[i]).and_modify(|ctr| *ctr += 1).or_insert(1);
                },
                _ => {
                    con += 1;
                }
            }

            while con > k {
                match word_ch[s] {
                    'a' | 'e' | 'i' | 'o' | 'u' => {
                        if let Some(&val) = vow.get(&word_ch[s]) {
                            if val == 1 {
                                vow.remove(&word_ch[s]);
                            } else {
                                vow.entry(word_ch[s]).and_modify(|ctr| *ctr -= 1);
                            }
                        }
                    },
                    _ => {
                        con -= 1;
                    }
                }
                s += 1;
            }

            while con == k && vow.len() == 5 {
                res += (next_con[i] - i) as i64;
                match word_ch[s] {
                    'a' | 'e' | 'i' | 'o' | 'u' => {
                        if let Some(&val) = vow.get(&word_ch[s]) {
                            if val == 1 {
                                vow.remove(&word_ch[s]);
                            } else {
                                vow.entry(word_ch[s]).and_modify(|ctr| *ctr -= 1);
                            }
                        }
                    },
                    _ => {
                        con -= 1;
                    }
                }
                s += 1;
            }
        }
        res
    }
}