// extern crate Q3_longest_substring_without_repeating_characters;

use Q3_longest_substring_without_repeating_characters::Solution;

fn main() {
    assert_eq!(3, Solution::length_of_longest_substring(String::from("abcabcbb")));
    assert_eq!(1, Solution::length_of_longest_substring(String::from("bbbbb")));
    assert_eq!(3, Solution::length_of_longest_substring(String::from("pwwkew")));
    assert_eq!(1, Solution::length_of_longest_substring(String::from(" ")));
}

