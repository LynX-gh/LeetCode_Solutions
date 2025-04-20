impl Solution {
    pub fn calculate_score(mut instructions: Vec<String>, values: Vec<i32>) -> i64 {
        let mut res = 0;
        let mut i = 0;
        let add_String = String::from("add");
        let jmp_String = String::from("jump");

        while i < instructions.len() {
            match instructions[i].as_str() {
                "add" => {
                    instructions[i].clear();
                    res += values[i] as i64;
                    i += 1;
                },
                "jump" => {
                    instructions[i].clear();
                    i += values[i] as usize;
                },
                _ => {
                    break;
                }
            }
        }
        res
    }
}
