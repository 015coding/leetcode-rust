use std::vec;

struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut ans: i32;
        let mut stack: Vec<i32> = Vec::new();
        let mut x: i32 = 0;
        let mut y: i32 = 0;

        for token in tokens {
            if matches!(token.as_str(), "+" | "-" | "*" | "/") {
                y = stack.pop().unwrap();
                x = stack.pop().unwrap();
            } else {
                stack.push(token.parse().unwrap());
            }

            match token.as_str() {
                "+" => {
                    ans = x + y;
                    stack.push(ans);
                }
                "-" => {
                    ans = x - y;
                    stack.push(ans);
                }
                "*" => {
                    ans = x * y;
                    stack.push(ans);
                }
                "/" => {
                    ans = x / y;
                    stack.push(ans);
                }
                _ => continue,
            }
        }
        ans = stack.pop().unwrap();
        ans
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::eval_rpn(
            vec!["4", "13", "5", "/", "+"]
                .into_iter()
                .map(String::from)
                .collect()
        )
    );
}
