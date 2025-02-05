use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        let bank_set: HashSet<_> = bank.into_iter().collect();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back((start_gene.clone(), 0));
        visited.insert(start_gene);

        while let Some((current_gene, moves)) = queue.pop_front() {
            if current_gene == end_gene {
                return moves;
            }

            for i in 0..current_gene.len() {
                for &c in &['A', 'C', 'G', 'T'] {
                    if c != current_gene.chars().nth(i).unwrap() {
                        let mut new_gene = current_gene.clone();
                        new_gene.replace_range(i..i+1, &c.to_string());

                        if bank_set.contains(&new_gene) && !visited.contains(&new_gene) {
                            visited.insert(new_gene.clone());
                            queue.push_back((new_gene, moves + 1));
                        }
                    }
                }
            }
        }

        -1
    }
}