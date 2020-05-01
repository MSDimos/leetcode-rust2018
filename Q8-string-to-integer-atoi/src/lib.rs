struct Solution {}

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let mut trimmed = str.trim_start().to_string();
        if (trimmed == "".to_string()) {
            return 0;
        }

        let start = trimmed.chars().next().unwrap();

        let mut signed: i64 = 1;
        let mut total: i64 = 0;

        if start == '+' {
            signed = 1;
        } else if start == '-' {
            signed = -1;
        } else if start.is_digit(10) {
            total += i64::from(start.to_digit(10).unwrap());
        } else {
            return 0;
        }

        trimmed = trimmed.chars().skip(1).collect();
        for i in trimmed.chars() {
            if i.is_digit(10) {
                total *= 10;
                total += signed * i64::from(i.to_digit(10).unwrap());
            } else {
                break;
            }

            if total > i64::from(i32::max_value()) {
                return i32::max_value();
            } else if total < i64::from(i32::min_value()) {
                return i32::min_value();
            }
        }
        return total as i32;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
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
}
