use std::cmp::max;

pub struct Solution {}

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        if prices.len() < 2 || k == 0 {
            return 0;
        }

        if k as usize >= prices.len() / 2 {
            return Solution::greedy(prices);
        }

        let k = k as usize + 1;
        let len = prices.len();
        let mut dp = vec![vec![vec![0; 2]; k]; len];

        for i in 0..k {
            dp[0][i][0] = 0;
            dp[0][i][1] = -prices[0];
        }

        for i in 1..len {
            for j in 0..k {
                if j == 0 {
                    dp[i][j][0] = dp[i - 1][j][0];
                } else {
                    dp[i][j][0] = max(dp[i - 1][j][0], dp[i - 1][j - 1][1] + prices[i]);
                }
                dp[i][j][1] = max(dp[i - 1][j][1], dp[i - 1][j][0] - prices[i]);
            }
        }

        let mut max_profit = i32::min_value();

        for i in 0..k {
            max_profit = max(max_profit, dp[len - 1][i][0]);
        }

        max_profit
    }

    fn greedy(prices: Vec<i32>) -> i32 {
        let mut max = 0;

        for i in 1..prices.len() {
            if prices[i] > prices[i - 1] {
                max += prices[i] - prices[i - 1];
            }
        }

        max
    }

    pub fn max_profit_s2(k: i32, prices: Vec<i32>) -> i32 {
        if prices.len() < 2 || k == 0 {
            return 0;
        }

        if k as usize >= prices.len() / 2 {
            return Solution::greedy(prices);
        }

        let k = k as usize;
        let mut dp = vec![vec![0; 2]; k];

        for i in 0..k {
            dp[i][0] = i32::min_value();
        }

        for price in prices {
            dp[0][0] = max(dp[0][0], -price);
            dp[0][1] = max(dp[0][1], dp[0][0] + price);

            for i in 1..k {
                // (i - 1)-th transaction is a sell, so u can buy stock at i-th transaction
                dp[i][0] = max(dp[i][0], dp[i - 1][1] - price);
                // i-th transaction is a but, so u can sell stock at i-th transaction
                dp[i][1] = max(dp[i][1], dp[i][0] + price);
            }
        }

        dp[k - 1][1]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::max_profit(2, vec![2, 4, 1]), 2);
        assert_eq!(Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]), 7);
    }

    #[test]
    fn it_works_s2() {
        assert_eq!(Solution::max_profit_s2(2, vec![2, 4, 1]), 2);
        assert_eq!(Solution::max_profit_s2(2, vec![3, 2, 6, 5, 0, 3]), 7);
        assert_eq!(Solution::max_profit_s2(2, vec![6, 1, 3, 2, 4, 7]), 7);
    }
}
