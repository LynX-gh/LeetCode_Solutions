use std::str::FromStr;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();
        let mut op1;
        let mut op2;
        
        for op in tokens {
            match op.as_str() {
                "+" => {
                    op1 = stack.pop().unwrap();
                    op2 = stack.pop().unwrap();
                    stack.push(op2 + op1);
                },
                "-" => {
                    op1 = stack.pop().unwrap();
                    op2 = stack.pop().unwrap();
                    stack.push(op2 - op1);
                },
                "*" => {
                    op1 = stack.pop().unwrap();
                    op2 = stack.pop().unwrap();
                    stack.push(op2 * op1);
                },
                "/" => {
                    op1 = stack.pop().unwrap();
                    op2 = stack.pop().unwrap();
                    stack.push(op2 / op1);
                },
                _ => {
                    stack.push(i32::from_str(op.as_str()).unwrap_or(0));
                }
            }
        }
        stack.pop().unwrap()
    }
}