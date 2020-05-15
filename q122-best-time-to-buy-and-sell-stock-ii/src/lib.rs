use std::cmp::max;

pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }

        let mut dp = vec![vec![0; 2]; prices.len()];

        dp[0][0] = 0;
        dp[0][1] = -prices[0];

        for i in 1..prices.len() {
            dp[i][0] = max(dp[i - 1][0], dp[i - 1][1] + prices[i]);
            dp[i][1] = max(dp[i - 1][1], dp[i - 1][0] - prices[i]);
        }

        dp[prices.len() - 1][0]
    }

    pub fn max_profit_s2(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }

        let mut stack = vec![];
        let mut max_profit = 0;

        for price in prices {
            if stack.is_empty() {
                stack.push(price);
            } else {
                if stack[stack.len() - 1] > price {
                    if stack.len() >= 2 {
                        max_profit += stack[stack.len() - 1] - stack[0];
                    }

                    stack.clear();
                }

                stack.push(price);
            }
        }

        if stack.len() >= 2 {
            max_profit += stack[stack.len() - 1] - stack[0];
        }

        max_profit
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn it_works_s2() {
        assert_eq!(Solution::max_profit_s2(vec![7, 1, 5, 3, 6, 4]), 7);
        assert_eq!(Solution::max_profit_s2(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit_s2(vec![1, 2, 3, 2, 1, 2, 3]), 4);
        assert_eq!(Solution::max_profit_s2(vec![7, 6, 4, 3, 1]), 0);
    }
}
