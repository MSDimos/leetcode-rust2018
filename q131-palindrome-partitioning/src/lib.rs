use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let s_bytes = s.into_bytes();
        let mut palindromes = HashSet::new();
        let mut result = vec![];
        let mut sub_strs = vec![];

        Self::dfs(&s_bytes[..], &mut sub_strs, &mut result, &mut palindromes);
        result
    }

    fn dfs<'a>(
        bytes: &'a [u8],
        sub_strs: &mut Vec<String>,
        result: &mut Vec<Vec<String>>,
        palindromes: &mut HashSet<&'a [u8]>,
    ) {
        if bytes.is_empty() {
            if !sub_strs.is_empty() {
                result.push(sub_strs.clone() as Vec<String>);
            }
        } else {
            for i in 0..bytes.len() {
                let sub_str = &bytes[..=i];

                if Self::is_palindrome(sub_str, palindromes) {
                    let segment = String::from_utf8(sub_str.to_vec()).unwrap();
                    sub_strs.push(segment);
                    Self::dfs(&bytes[(i + 1)..], sub_strs, result, palindromes);
                    sub_strs.pop();
                }
            }
        }
    }

    fn is_palindrome<'a>(bytes: &'a [u8], palindromes: &mut HashSet<&'a [u8]>) -> bool {
        !bytes.is_empty()
            && (palindromes.contains(bytes) || {
                let (mut left, mut right) = (0, bytes.len() - 1);

                while left <= right && right > 0 {
                    if bytes[left] == bytes[right] {
                        left += 1;
                        right -= 1;
                    } else {
                        return false;
                    }
                }

                palindromes.insert(bytes);
                true
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let input = "aab".to_string();
        assert_eq!(
            Solution::partition(input),
            vec![
                vec!["a".to_string(), "a".to_string(), "b".to_string()],
                vec!["aa".to_string(), "b".to_string()],
            ],
        );

        let input = "abc".to_string();
        assert_eq!(
            Solution::partition(input),
            vec![vec!["a".to_string(), "b".to_string(), "c".to_string()],],
        );

        let input = "a".to_string();
        assert_eq!(Solution::partition(input), vec![vec!["a".to_string()]]);

        let input = "".to_string();
        let output: Vec<Vec<String>> = vec![];
        assert_eq!(Solution::partition(input), output);
    }
}
