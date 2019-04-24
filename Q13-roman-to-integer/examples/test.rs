use Q13_roman_to_integer::Solution;

fn main() {
    assert_eq!(4, Solution::roman_to_int(String::from("IV")));
    assert_eq!(3, Solution::roman_to_int(String::from("III")));
    assert_eq!(58, Solution::roman_to_int(String::from("LVIII")));
    assert_eq!(1994, Solution::roman_to_int(String::from("MCMXCIV")));
    assert_eq!(1, Solution::roman_to_int(String::from("I")));
    assert_eq!(0, Solution::roman_to_int(String::from("")));
}