use std::collections::{VecDeque, HashSet, HashMap};

impl Solution {
    pub fn find_all_recipes(recipes: Vec<String>, ingredients: Vec<Vec<String>>, supplies: Vec<String>) -> Vec<String> {
        let mut indeg: HashMap<String, usize> = HashMap::new();
        let mut ingredient_to_recipes: HashMap<String, Vec<String>> = HashMap::new();
        let mut res = Vec::new();
        let mut added: HashSet<String> = supplies.iter().cloned().collect();
        let mut queue: VecDeque<String> = VecDeque::from(supplies);

        // Initialize ingredient dependency count and reverse dependency map
        for (i, recipe) in recipes.iter().enumerate() {
            let count = ingredients[i].len();
            indeg.insert(recipe.clone(), count);
            for ingredient in &ingredients[i] {
                ingredient_to_recipes.entry(ingredient.clone()).or_default().push(recipe.clone());
            }
        }

        while let Some(supply) = queue.pop_front() {
            if let Some(dependents) = ingredient_to_recipes.get(&supply) {
                for dependent in dependents {
                    if let Some(count) = indeg.get_mut(dependent) {
                        *count -= 1;
                        if *count == 0 && added.insert(dependent.clone()) {
                            queue.push_back(dependent.clone());
                            res.push(dependent.clone());
                        }
                    }
                }
            }
        }

        res
    }
}