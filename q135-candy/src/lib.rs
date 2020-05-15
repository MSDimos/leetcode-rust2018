use std::cmp::max;

pub struct Solution {}

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        if ratings.len() <= 1 {
            return ratings.len() as i32;
        }

        let len = ratings.len();
        let mut candies = vec![1; len];
        let mut sum = 0;

        // from left to right, if one get higher rating than previous,
        // then add one by candies previous child can get
        // for instance, if ratings are [1, 2, 3, 2, 1], candies after loop are [1, 2, 3, 1, 1]
        // u can notice that penultimate number of candy is incorrect
        // so we also need to traverse on the contrary and fix those errors
        for i in 1..len {
            if ratings[i] > ratings[i - 1] {
                candies[i] = candies[i - 1] + 1;
            }
        }

        // fix errors mentioned above
        for i in (0..=(len - 2)).rev() {
            if ratings[i] > ratings[i + 1] {
                candies[i] = max(candies[i], candies[i + 1] + 1);
            }
        }

        for candy in candies {
            sum += candy;
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let input = vec![1, 0, 2];
        assert_eq!(Solution::candy(input), 5);

        let input = vec![1, 2, 2];
        assert_eq!(Solution::candy(input), 4);

        let input = vec![0];
        assert_eq!(Solution::candy(input), 1);
    }
}
