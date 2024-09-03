impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut res = Vec::with_capacity(n.try_into().unwrap());
        for num in 1..=n {
            if num % 15 == 0 {
                res.push(String::from("FizzBuzz"));
            }
            else if num % 3 == 0 {
                res.push(String::from("Fizz"));
            }
            else if num % 5 == 0 {
                res.push(String::from("Buzz"));
            }
            else {
                res.push(num.to_string());
            }
        }
        res
    }
}