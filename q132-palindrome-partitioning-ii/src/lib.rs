use std::cmp::min;
use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn min_cut_s1(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let bytes = s.into_bytes();
        let mut palindromes = HashSet::new();
        let mut dp = vec![bytes.len() as i32; bytes.len() + 1];

        dp[0] = -1;

        for i in 0..bytes.len() {
            for j in 0..=i {
                if Self::is_palindrome(&bytes[j..=i], &mut palindromes) {
                    dp[i + 1] = min(dp[i + 1], dp[j] + 1);
                }
            }
        }

        *dp.last().unwrap()
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

    pub fn min_cut_s2(s: String) -> i32 {
        if s.len() <= 1 {
            return 0;
        }

        let bytes = s.into_bytes();
        let mut dp = vec![bytes.len() as i32 - 1; bytes.len()];

        for i in 0..bytes.len() as i32 {
            Self::min_cut_helper_s2(&bytes, (i, i), &mut dp);
            Self::min_cut_helper_s2(&bytes, (i, i + 1), &mut dp);
        }

        dp[bytes.len() - 1]
    }

    fn min_cut_helper_s2(bytes: &[u8], (mut i, mut j): (i32, i32), dp: &mut [i32]) {
        while i >= 0 && j < bytes.len() as i32 && bytes[i as usize] == bytes[j as usize] {
            if i == 0 {
                dp[j as usize] = min(dp[j as usize], 0);
            } else {
                dp[j as usize] = min(dp[j as usize], dp[i as usize - 1] + 1);
            }
            i -= 1;
            j += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works_s1() {
        let input = "aab".to_string();
        assert_eq!(Solution::min_cut_s1(input), 1);

        let input = "abc".to_string();
        assert_eq!(Solution::min_cut_s1(input), 2);

        let input = "a".to_string();
        assert_eq!(Solution::min_cut_s1(input), 0);

        let input = "".to_string();
        assert_eq!(Solution::min_cut_s1(input), 0);

        let input = "aaabaa".to_string();
        assert_eq!(Solution::min_cut_s1(input), 1);

        let input = "ababababababababababababcbabababababababababababa".to_string();
        assert_eq!(Solution::min_cut_s1(input), 0);

        let input = "a".repeat(150);
        assert_eq!(Solution::min_cut_s1(input), 0);
    }

    #[test]
    fn it_works_s2() {
        let input = "aab".to_string();
        assert_eq!(Solution::min_cut_s2(input), 1);

        let input = "abc".to_string();
        assert_eq!(Solution::min_cut_s2(input), 2);

        let input = "a".to_string();
        assert_eq!(Solution::min_cut_s2(input), 0);

        let input = "".to_string();
        let output: Vec<Vec<String>> = vec![];
        assert_eq!(Solution::min_cut_s2(input), 0);

        let input = "aaabaa".to_string();
        assert_eq!(Solution::min_cut_s2(input), 1);

        let input = "ababababababababababababcbabababababababababababa".to_string();
        assert_eq!(Solution::min_cut_s2(input), 0);

        let input = "a".repeat(1426);
        assert_eq!(Solution::min_cut_s2(input), 0);
    }
}
