use Q9_palindrome_number::Solution;

fn main() {
    assert_eq!(true, Solution::is_palindrome(1221));
    assert_eq!(false, Solution::is_palindrome(-121));
    assert_eq!(false, Solution::is_palindrome(10));
}