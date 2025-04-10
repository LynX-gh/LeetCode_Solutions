impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let n = score.len();
        let mut athletes: Vec<(usize, i32)> = score
            .into_iter()
            .enumerate()
            .collect();
        
        athletes.sort_unstable_by_key(|x| -x.1);

        let mut result = vec![String::new(); n];
        
        for (rank, athlete) in athletes.iter().enumerate() {
            result[athlete.0] = match rank {
                0 => "Gold Medal".to_string(),
                1 => "Silver Medal".to_string(),
                2 => "Bronze Medal".to_string(),
                _ => (rank + 1).to_string()
            };
        }

        result
    }
}
