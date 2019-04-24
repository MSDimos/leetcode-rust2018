use Q5_longest_palindromic_substring::{ Solution, expand_around_center };

fn main() {
    let s = Solution::longest_palindrome(String::from("babad"));

    assert_eq!("bab", &s);
}