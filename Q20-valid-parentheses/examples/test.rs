use Q20_valid_parentheses::Solution;

fn main() {
    assert!(Solution::is_valid(String::from("()")));
    assert!(Solution::is_valid(String::from("()[]{}")));
    assert!(!Solution::is_valid(String::from("(]")));
    assert!(!Solution::is_valid(String::from("([)]")));
    assert!(Solution::is_valid(String::from("{[]}")));
}