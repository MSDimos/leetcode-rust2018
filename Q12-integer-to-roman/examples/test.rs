use Q12_integer_to_roman::Solution;

fn main() {
    assert_eq!(String::from("III"), Solution::int_to_roman(3));
    assert_eq!(String::from("IV"), Solution::int_to_roman(4));
    assert_eq!(String::from("IX"), Solution::int_to_roman(9));
    assert_eq!(String::from("LVIII"), Solution::int_to_roman(58));
    assert_eq!(String::from("MCMXCIV"), Solution::int_to_roman(1994));
    assert_eq!(String::from("X"), Solution::int_to_roman(10));
}