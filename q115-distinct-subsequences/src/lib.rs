pub struct Solution {}

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s_bytes = s.into_bytes();
        let t_bytes = t.into_bytes();
        let (m, n) = (s_bytes.len(), t_bytes.len());
        let mut dp = vec![vec![0; m + 1]; n + 1];

        for j in 0..(m + 1) {
            dp[0][j] = 1;
        }

        for i in 1..=n {
            for j in i..=m {
                // if t_bytes[i - 1] == s_bytes[j - 1], there are two options
                // 1. consume s_bytes[j - 1]
                // 2. don't consume it
                dp[i][j] = if t_bytes[i - 1] == s_bytes[j - 1] {
                    dp[i - 1][j - 1] + dp[i][j - 1]
                } else {
                    dp[i][j - 1]
                }
            }
        }

        dp[n][m]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::num_distinct("rabbbit".to_string(), "rabbit".to_string()),
            3
        );
        assert_eq!(
            Solution::num_distinct("babgbag".to_string(), "bag".to_string()),
            5
        );
    }
}
