use Q8_string_to_integer_atoi::Solution;

fn main() {
    assert_eq!(42, Solution::my_atoi(String::from("42")));
    assert_eq!(-42, Solution::my_atoi(String::from("    -42")));
    assert_eq!(-2147483648, Solution::my_atoi(String::from("-91283472332")));
    assert_eq!(0, Solution::my_atoi(String::from("words and 987")));
    assert_eq!(3, Solution::my_atoi(String::from("3.14159")));
    assert_eq!(-3, Solution::my_atoi(String::from("-3.14159")));
    assert_eq!(-3, Solution::my_atoi(String::from("  -3.14159")));
    assert_eq!(0, Solution::my_atoi(String::from("+ 0 123")));
    assert_eq!(0, Solution::my_atoi(String::from("-")));
}