pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.is_empty() {
            return true;
        }

        let s_bytes = s.into_bytes();
        let (mut left, mut right) = (0, s_bytes.len() - 1);

        while left < right {
            while left < s_bytes.len() && !Solution::is_valid_char(s_bytes[left]) {
                left += 1;
            }

            while right > 0 && !Solution::is_valid_char(s_bytes[right]) {
                right -= 1;
            }

            if left <= right && right > 0 {
                if s_bytes[left].to_ascii_lowercase() == s_bytes[right].to_ascii_lowercase() {
                    left += 1;
                    right -= 1;
                } else {
                    return false;
                }
            }
        }

        true
    }

    fn is_valid_char(chr: u8) -> bool {
        (chr >= b'a' && chr <= b'z') || (chr >= b'A' && chr <= b'Z') || (chr >= b'0' && chr <= b'9')
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let input = "A man, a plan, a canal: Panama".to_string();
        assert_eq!(Solution::is_palindrome(input), true);

        let input = "race a car".to_string();
        assert_eq!(Solution::is_palindrome(input), false);

        let input = "".to_string();
        assert_eq!(Solution::is_palindrome(input), true);

        let input = "a.".to_string();
        assert_eq!(Solution::is_palindrome(input), true);

        let input = "a".to_string();
        assert_eq!(Solution::is_palindrome(input), true);

        let input = "aba".to_string();
        assert_eq!(Solution::is_palindrome(input), true);

        let input = " a b  a   ".to_string();
        assert_eq!(Solution::is_palindrome(input), true);
    }
}
