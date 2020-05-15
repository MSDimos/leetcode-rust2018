use std::cmp::max;

pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }

        let len = prices.len();
        let mut dp = vec![vec![vec![0; 2]; 3]; len];

        for i in 0..3 {
            dp[0][i][0] = 0;
            dp[0][i][1] = -prices[0];
        }

        for i in 1..len {
            for j in 0..3 {
                if j == 0 {
                    dp[i][j][0] = dp[i - 1][j][0];
                } else {
                    dp[i][j][0] = max(dp[i - 1][j][0], dp[i - 1][j - 1][1] + prices[i]);
                }

                dp[i][j][1] = max(dp[i - 1][j][1], dp[i - 1][j][0] - prices[i]);
            }
        }

        max(dp[len - 1][1][0], dp[len - 1][2][0])
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(Solution::max_profit(vec![]), 0);
    }
}
