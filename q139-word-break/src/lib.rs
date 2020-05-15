use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        if s.is_empty() {
            return word_dict.is_empty();
        }

        let bytes = s.into_bytes();
        let mut hash_set = HashSet::new();
        let mut dp = vec![false; bytes.len() + 1];

        for word in word_dict {
            hash_set.insert(word.into_bytes());
        }

        dp[0] = true;

        for i in 1..=bytes.len() {
            for j in 0..i {
                if dp[j] && hash_set.contains(&bytes[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }

        dp[bytes.len()]
    }

    pub fn word_break_s2(s: String, word_dict: Vec<String>) -> bool {
        if s.is_empty() {
            return word_dict.is_empty();
        }

        let mut mem = vec![None; s.len()];
        let mut dict = HashSet::new();

        for word in word_dict {
            dict.insert(word.into_bytes());
        }

        Self::helper(s.as_bytes(), &dict, 0, &mut mem)
    }

    fn helper(
        s: &[u8],
        word_dict: &HashSet<Vec<u8>>,
        start: usize,
        mem: &mut [Option<bool>],
    ) -> bool {
        if start == s.len() {
            return true;
        }

        if mem[start].is_some() {
            return mem[start].unwrap();
        }

        for end in (start + 1)..=s.len() {
            if word_dict.contains(&s[start..end]) && Self::helper(s, word_dict, end, mem) {
                mem[end - 1] = Some(true);
                return true;
            }
        }

        mem[start] = Some(false);
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let s = "leetcode".to_string();
        let word_dict = vec!["leet".to_string(), "code".to_string()];
        assert_eq!(Solution::word_break(s, word_dict), true);

        let s = "applepenapple".to_string();
        let word_dict = vec!["apple".to_string(), "pen".to_string()];
        assert_eq!(Solution::word_break(s, word_dict), true);

        let s = "catsandog".to_string();
        let word_dict = vec![
            "cats".to_string(),
            "dog".to_string(),
            "sand".to_string(),
            "and".to_string(),
            "cat".to_string(),
        ];
        assert_eq!(Solution::word_break(s, word_dict), false);

        let s = "".to_string();
        let word_dict = vec!["apple".to_string(), "pen".to_string()];
        assert_eq!(Solution::word_break(s, word_dict), false);

        let s = "".to_string();
        let word_dict = vec![];
        assert_eq!(Solution::word_break(s, word_dict), true);

        let s = "abcd".to_string();
        let word_dict = vec![
            "a".to_string(),
            "abc".to_string(),
            "b".to_string(),
            "cd".to_string(),
        ];
        assert_eq!(Solution::word_break(s, word_dict), true);

        let s = "asas".to_string();
        let word_dict = vec![];
        assert_eq!(Solution::word_break(s, word_dict), false);
    }

    #[test]
    fn it_works_s2() {
        let s = "leetcode".to_string();
        let word_dict = vec!["leet".to_string(), "code".to_string()];
        assert_eq!(Solution::word_break_s2(s, word_dict), true);

        let s = "applepenapple".to_string();
        let word_dict = vec!["apple".to_string(), "pen".to_string()];
        assert_eq!(Solution::word_break_s2(s, word_dict), true);

        let s = "catsandog".to_string();
        let word_dict = vec![
            "cats".to_string(),
            "dog".to_string(),
            "sand".to_string(),
            "and".to_string(),
            "cat".to_string(),
        ];
        assert_eq!(Solution::word_break_s2(s, word_dict), false);

        let s = "".to_string();
        let word_dict = vec!["apple".to_string(), "pen".to_string()];
        assert_eq!(Solution::word_break_s2(s, word_dict), false);

        let s = "".to_string();
        let word_dict = vec![];
        assert_eq!(Solution::word_break_s2(s, word_dict), true);

        let s = "abcd".to_string();
        let word_dict = vec![
            "a".to_string(),
            "abc".to_string(),
            "b".to_string(),
            "cd".to_string(),
        ];
        assert_eq!(Solution::word_break_s2(s, word_dict), true);

        let s = "asas".to_string();
        let word_dict = vec![];
        assert_eq!(Solution::word_break_s2(s, word_dict), false);
    }
}
