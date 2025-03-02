impl Solution {
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        let klen = k as usize;
        let mut freq = vec![vec![]; 51];

        for i in 0..=nums.len()-klen {
            let mut seen = vec![false; 51];
            for j in i..i+klen {
                seen[nums[j] as usize] = true;
            }
            for num in 0..51 {
                if seen[num] {
                    freq[num].push(i);
                }
            }
        }
        for i in (0..51).rev() {
            if freq[i].len() == 1 {
                return i as i32;
            }
        }
        
        -1
    }
}