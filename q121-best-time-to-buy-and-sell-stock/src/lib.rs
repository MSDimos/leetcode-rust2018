use std::cmp::{max, min};

pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_pirce = i32::max_value();
        let mut max_profit = 0;

        for price in prices {
            min_pirce = min(price, min_pirce);
            max_profit = max(max_profit, price - min_pirce);
        }

        max_profit
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(Solution::max_profit(vec![1]), 0);
        assert_eq!(Solution::max_profit(vec![1, 2]), 1);
        assert_eq!(Solution::max_profit(vec![]), 0);
        assert_eq!(Solution::max_profit(vec![3, 2, 1]), 0);
        assert_eq!(Solution::max_profit(vec![2, 1]), 0);
        assert_eq!(Solution::max_profit(vec![4, 3, 2, 1]), 0);
        assert_eq!(Solution::max_profit(vec![1, 2, 3]), 2);
    }
}
