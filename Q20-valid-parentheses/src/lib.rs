pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let left_parentheses = ['[', '(', '{'];
        let right_parentheses = [']', ')', '}'];
        let mut stack = vec![];

        for chr in s.chars() {
            if left_parentheses.contains(&chr) {
                stack.push(chr);
            } else {
                if stack.is_empty() {
                    return false;
                } else {
                    let idx = right_parentheses.iter().position(|c| c == &chr).unwrap();
                    let top = stack.pop().unwrap();

                    if top != left_parentheses[idx] {
                        return false;
                    }
                }
            }
        }

//        another solution
//        while s.contains("()") || s.contains("[]") || s.contains("{}") {
//            s.replace("()", "");
//            s.replace("[]", "");
//            s.replace("{}", "");
//        }
//        s.len() == 0

        stack.is_empty()
    }
}