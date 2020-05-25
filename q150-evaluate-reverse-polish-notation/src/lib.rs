pub struct Solution {}

#[derive(Debug)]
enum Token {
    Number(i32),
    Ops(String),
}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut new_tokens = vec![];
        let mut result = 0;

        for token in tokens.into_iter().rev() {
            let new_token = match token.as_str() {
                "+" | "-" | "*" | "/" => Token::Ops(token.to_string()),
                _ => Token::Number(token.parse().unwrap_or(0)),
            };

            new_tokens.push(new_token);
        }

        let mut stack = vec![];

        while let Some(tmp_token) = new_tokens.pop() {
            match tmp_token {
                Token::Number(num) => stack.push(num),
                Token::Ops(op) => {
                    let num1 = stack.pop().unwrap_or(0);
                    let num2 = stack.pop().unwrap_or(0);
                    let op_result = match op.as_str() {
                        "+" => num2 + num1,
                        "-" => num2 - num1,
                        "*" => num2 * num1,
                        "/" => num2 / num1,
                        _ => 0,
                    };

                    new_tokens.push(Token::Number(op_result));
                    result = op_result;

                    while !stack.is_empty() {
                        new_tokens.push(Token::Number(stack.pop().unwrap()));
                    }
                }
            }
        }

        if stack.is_empty() {
            result
        } else {
            stack[0]
        }
    }

    pub fn eval_rpn_s2(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];

        for token in tokens {
            match token.as_str() {
                "+" | "-" | "*" | "/" => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();

                    stack.push(Self::op(token.as_str(), a, b));
                }
                num => stack.push(num.parse::<i32>().unwrap()),
            }
        }

        stack.pop().unwrap()
    }

    fn op(op: &str, a: i32, b: i32) -> i32 {
        match op {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => a / b,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let input: Vec<String> = ["2", "1", "+", "3", "*"]
            .to_vec()
            .into_iter()
            .map(|a| String::from(a))
            .collect();
        assert_eq!(Solution::eval_rpn(input), 9);

        let input: Vec<String> = ["4", "13", "5", "/", "+"]
            .to_vec()
            .into_iter()
            .map(|a| String::from(a))
            .collect();
        assert_eq!(Solution::eval_rpn(input), 6);

        let input: Vec<String> = [
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ]
        .to_vec()
        .into_iter()
        .map(|a| String::from(a))
        .collect();
        assert_eq!(Solution::eval_rpn(input), 22);

        let input: Vec<String> = ["10"]
            .to_vec()
            .into_iter()
            .map(|a| String::from(a))
            .collect();
        assert_eq!(Solution::eval_rpn(input), 10);
    }

    #[test]
    fn it_works_s2() {
        let input: Vec<String> = ["2", "1", "+", "3", "*"]
            .to_vec()
            .into_iter()
            .map(|a| String::from(a))
            .collect();
        assert_eq!(Solution::eval_rpn_s2(input), 9);

        let input: Vec<String> = ["4", "13", "5", "/", "+"]
            .to_vec()
            .into_iter()
            .map(|a| String::from(a))
            .collect();
        assert_eq!(Solution::eval_rpn_s2(input), 6);

        let input: Vec<String> = [
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ]
        .to_vec()
        .into_iter()
        .map(|a| String::from(a))
        .collect();
        assert_eq!(Solution::eval_rpn_s2(input), 22);

        let input: Vec<String> = ["10"]
            .to_vec()
            .into_iter()
            .map(|a| String::from(a))
            .collect();
        assert_eq!(Solution::eval_rpn_s2(input), 10);
    }
}
