struct Solution {}

impl Solution {
    pub fn is_interleave_s1(s1: String, s2: String, s3: String) -> bool {
        // solution 1, slow but pass
        let target = s3.into_bytes();
        let s1_bytes = s1.into_bytes();
        let s2_bytes = s2.into_bytes();

        Solution::helper(&target[..], &s1_bytes[..], &s2_bytes[..])
    }

    pub fn is_interleave_s2(s1: String, s2: String, s3: String) -> bool {
        // solution 2, dynamic programming
        let target = s3.into_bytes();
        let s1_bytes = s1.into_bytes();
        let s2_bytes = s2.into_bytes();

        if target.len() != s1_bytes.len() + s2_bytes.len() {
            return false;
        }

        let mut dp = vec![vec![false; s2_bytes.len() + 1]; s1_bytes.len() + 1];

        for i in 0..=s1_bytes.len() {
            for j in 0..=s2_bytes.len() {
                if i == 0 && j == 0 {
                    dp[i][j] = true;
                } else if i == 0 {
                    dp[i][j] = dp[i][j - 1] && s2_bytes[j - 1] == target[i + j - 1];
                } else if j == 0 {
                    dp[i][j] = dp[i - 1][j] && s1_bytes[i - 1] == target[i + j - 1];
                } else {
                    let s1_bool = dp[i - 1][j] && s1_bytes[i - 1] == target[i + j - 1];
                    let s2_bool = dp[i][j - 1] && s2_bytes[j - 1] == target[i + j - 1];

                    dp[i][j] = s1_bool || s2_bool;
                }
            }
        }

        dp[s1_bytes.len()][s2_bytes.len()]
    }

    fn helper(target: &[u8], s1_bytes: &[u8], s2_bytes: &[u8]) -> bool {
        if target.is_empty() && s1_bytes.is_empty() && s2_bytes.is_empty() {
            true
        } else if target.len() != s1_bytes.len() + s2_bytes.len() {
            false
        } else if s1_bytes.is_empty() || s2_bytes.is_empty() {
            if s1_bytes.is_empty() {
                target == s2_bytes
            } else {
                target == s1_bytes
            }
        } else {
            if s1_bytes[0] == s2_bytes[0] {
                if s1_bytes[0] == target[0] {
                    Solution::helper(&target[1..], &s1_bytes[1..], s2_bytes) ||
                        Solution::helper(&target[1..], &s1_bytes, &s2_bytes[1..])
                } else {
                    false
                }
            } else if s1_bytes[0] == target[0] {
                Solution::helper(&target[1..], &s1_bytes[1..], s2_bytes)
            } else if s2_bytes[0] == target[0] {
                Solution::helper(&target[1..], &s1_bytes, &s2_bytes[1..])
            } else {
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn solution1_test() {
        assert_eq!(Solution::is_interleave_s1("a".to_string(), "a".to_string(), "aa".to_string()), true);
        assert_eq!(Solution::is_interleave_s1("aabcc".to_string(), "dbbca".to_string(), "aadbbcbcac".to_string()), true);
        assert_eq!(Solution::is_interleave_s1("aabcc".to_string(), "dbbca".to_string(), "aadbbbaccc".to_string()), false);
        assert_eq!(Solution::is_interleave_s1("".to_string(), "a".to_string(), "a".to_string()), true);
        assert_eq!(Solution::is_interleave_s1("a".to_string(), "".to_string(), "a".to_string()), true);
        assert_eq!(Solution::is_interleave_s1("a".to_string(), "".to_string(), "".to_string()), false);
    }

    #[test]
    fn solution2_test() {
        assert_eq!(Solution::is_interleave_s2("a".to_string(), "a".to_string(), "aa".to_string()), true);
        assert_eq!(Solution::is_interleave_s2("aabcc".to_string(), "dbbca".to_string(), "aadbbcbcac".to_string()), true);
        assert_eq!(Solution::is_interleave_s2("aabcc".to_string(), "dbbca".to_string(), "aadbbbaccc".to_string()), false);
        assert_eq!(Solution::is_interleave_s2("".to_string(), "a".to_string(), "a".to_string()), true);
        assert_eq!(Solution::is_interleave_s2("a".to_string(), "".to_string(), "a".to_string()), true);
        assert_eq!(Solution::is_interleave_s2("a".to_string(), "".to_string(), "".to_string()), false);
    }
}
