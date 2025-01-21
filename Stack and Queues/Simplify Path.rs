impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = Vec::new();
        for word in path.split('/') {
            match word {
                "" | " " | "." => (),
                ".." => {
                    stack.pop();
                },
                _ => {
                    stack.push(word.clone());
                },
            };
        }
        format!("/{}", stack.join("/"))
    }
}