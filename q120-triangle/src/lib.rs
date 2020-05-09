use std::cmp::min;

pub struct Solution {}

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        if triangle.is_empty() {
            return 0;
        }

        let row = triangle.len();
        let col = triangle.last().unwrap().len();
        let mut dp = vec![vec![0; col]; row];

        dp[0][0] = triangle[0][0];

        for i in 1..row {
            for j in 0..=i {
                if j == 0 {
                    // most left
                    dp[i][j] = dp[i - 1][j] + triangle[i][j];
                } else if i == j {
                    // most right
                    dp[i][j] = dp[i - 1][j - 1] + triangle[i][j];
                } else {
                    dp[i][j] = min(dp[i - 1][j], dp[i - 1][j - 1]) + triangle[i][j];
                }
            }
        }

        let mut min_sum = i32::max_value();

        for j in 0..col {
            min_sum = min(min_sum, dp[row - 1][j]);
        }

        min_sum
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let input = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        assert_eq!(Solution::minimum_total(input), 11);
    }
}
