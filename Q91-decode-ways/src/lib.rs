struct Solution {}

impl Solution {
    // dp[i] = dp[i - 1] + dp[i - 2], four situations
    // 1. s[i] == '0', if s[i - 1] == '1' || '2', dp[i] =d[i - 2], otherwise return 0;
    //    s[i - 1] + s[i] is the only valid ways
    // 2. s[i - 1] == '1', dp[i] = dp[i - 1] + dp[i - 2]
    // 3. s[i - 1] == '2' and '1' <= s[i] <= '6', dp[i] = dp[i - 1] + dp[i + 2]
    // 4. s[i] != '0' && s[i - 1] != '1' || '2', dp[i] = dp[i - 1]
    pub fn num_decodings(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let mut dp = vec![0; s.len() + 2];
        let mut s_bytes = s.into_bytes();
        // default value, preset two values so as not to index with overflow
        dp[0] = 1;
        dp[1] = 1;

        for i in 0..s_bytes.len() {
            let idx = i + 2;

            if i > 0 {
                if s_bytes[i] == b'0' {
                    if s_bytes[i - 1] == b'1' || s_bytes[i - 1] == b'2' {
                        dp[idx] = dp[idx - 2];
                    } else {
                        // error, can't match
                        return 0;
                    }
                } else if s_bytes[i - 1] == b'1' {
                    dp[idx] = dp[idx - 1] + dp[idx - 2];
                } else if s_bytes[i - 1] == b'2' && (s_bytes[i] >= b'1' && s_bytes[i] <= b'6') {
                    dp[idx] = dp[idx - 1] + dp[idx - 2];
                } else {
                    dp[idx] = dp[idx - 1];
                }
            } else {
                if s_bytes[i] == b'0' {
                    return 0;
                } else {
                    dp[idx] = dp[idx - 1];
                }
            }
        }

        dp[s_bytes.len() + 1]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example_test() {
        assert_eq!(Solution::num_decodings("12".to_string()), 2);
        assert_eq!(Solution::num_decodings("226".to_string()), 3);
        assert_eq!(Solution::num_decodings("102036".to_string()), 1);
        assert_eq!(Solution::num_decodings("102016".to_string()), 2);
        assert_eq!(Solution::num_decodings("111".to_string()), 3);
        assert_eq!(Solution::num_decodings("".to_string()), 0);
        assert_eq!(Solution::num_decodings("000".to_string()), 0);
        assert_eq!(Solution::num_decodings("100".to_string()), 0);
        assert_eq!(Solution::num_decodings("123456".to_string()), 3);
        assert_eq!(Solution::num_decodings("0".to_string()), 0);
    }
}
