pub struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut tokens = Self::read_tokens(s);
        let len = tokens.len();
        let mut result = String::new();

        for (idx, token) in tokens.into_iter().rev().enumerate() {
            result.push_str(&token);

            if idx != len - 1 {
                result.push_str(" ");
            }
        }

        result
    }

    fn read_tokens(s: String) -> Vec<String> {
        let bytes = s.into_bytes();
        let mut token = vec![];
        let mut result = vec![];

        for byte in bytes.into_iter() {
            if byte == b' ' {
                if token.is_empty() {
                    continue;
                } else {
                    result.push(String::from_utf8(token.to_vec()).unwrap());
                    token.clear();
                }
            } else {
                token.push(byte);
            }
        }

        if !token.is_empty() {
            result.push(String::from_utf8(token.to_vec()).unwrap());
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let input = "the sky is blue".to_string();
        assert_eq!(
            Solution::reverse_words(input),
            "blue is sky the".to_string()
        );

        let input = "  hello world!  ".to_string();
        assert_eq!(Solution::reverse_words(input), "world! hello".to_string());

        let input = "a good   example".to_string();
        assert_eq!(Solution::reverse_words(input), "example good a".to_string());
    }
}
