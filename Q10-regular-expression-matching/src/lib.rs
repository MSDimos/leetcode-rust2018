pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s_bytes = s.into_bytes();
        let p_bytes = p.into_bytes();
        let s_len = s_bytes.len();
        let p_len = p_bytes.len();
        // dp[i][j]代表s[0..i]与p[0..j]的匹配情况
        let mut dp = vec![ vec![false; p_len + 1]; s_len + 1 ];

        // 空字符串一定匹配空字符串
        dp[0][0] = true;

        for i in 0..(s_len + 1) {
            for j in 1..(p_len + 1) {
                if p_bytes[j - 1] == b'*' {
                    // 匹配0次或1次以上
                    // 0次: dp[i][j] = dp[i][j - 2]， 说明.*不影响原来的匹配
                    // 1次： 如果d[i - 1][j]已经匹配了，此时可能已经多次匹配或者还没匹配过，则检查两个字母是否相等或者p是否为'.'
                    dp[i][j] = dp[i][j - 2] || (i > 0 && dp[i - 1][j] && (s_bytes[i - 1] == p_bytes[j - 2] || p_bytes[j - 2] == b'.'))
                } else {
                    dp[i][j] = i > 0 && dp[i - 1][j - 1] && (s_bytes[i - 1] == p_bytes[j - 1] || p_bytes[j - 1] == b'.');
                }
            }
        }

        dp[s_len][p_len]
    }
}