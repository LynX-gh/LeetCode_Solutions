impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let mut res = 0;
        let mut window = 0;
        let sch: Vec<char> = s.chars().collect();
        let klen = k as usize;

        for i in 0..klen {
            match sch[i] {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    window += 1;
                },
                _ => ()
            }
        }
        res = window;

        for i in 0..sch.len() - klen {
            match sch[i] {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    window -= 1;
                },
                _ => ()
            }
            match sch[i + klen] {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    window += 1;
                },
                _ => ()
            }
            res = res.max(window);
        }
        res
        
    }
}