
pub struct Solution;

fn dsp(i: i32, total: i32, stack: &mut String, left_parentheses: i32, res: &mut Vec<String>) {
    if i == total {
        if left_parentheses == 0 {
            res.push(stack.clone());
        }
    } else {
        stack.push('(');
        dsp(i + 1, total, stack, left_parentheses + 1, res);
        stack.pop();

        if left_parentheses > 0 {
            stack.push(')');
            dsp(i + 1, total, stack, left_parentheses - 1, res);
            stack.pop();
        }

    }
}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = vec![];
        let mut stack = String::new();

        dsp(0, 2 * n, &mut stack, 0, &mut res);

        res
    }
}