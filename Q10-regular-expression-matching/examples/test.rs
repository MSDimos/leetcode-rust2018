use Q10_regular_expression_matching::Solution;

fn main() {
    assert_eq!(true, Solution::is_match(String::from("aa"), String::from("a*")));
    assert_eq!(false, Solution::is_match(String::from("aa"), String::from("a")));
    assert_eq!(true, Solution::is_match(String::from("ab"), String::from(".*")));
    assert_eq!(true, Solution::is_match(String::from("aab"), String::from("c*a*b")));
    assert_eq!(false, Solution::is_match(String::from("mississippi"), String::from("mis*is*p*.")));
}