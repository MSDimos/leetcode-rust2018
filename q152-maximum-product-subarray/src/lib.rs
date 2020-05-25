use std::cmp::{max, min};

pub struct Solution {}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max_product = nums[0];
        let mut min_product = nums[0];
        let mut result = nums[0];

        for num in nums.into_iter().skip(1) {
            let min_tmp = min_product * num;
            let max_tmp = max_product * num;

            max_product = max(num, max(min_tmp, max_tmp));
            min_product = min(num, min(min_tmp, max_tmp));
            result = max(max_product, result);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let input = vec![2, 3, -2, 4];
        assert_eq!(Solution::max_product(input), 6);

        let input = vec![-2, 0, -1];
        assert_eq!(Solution::max_product(input), 0);

        let input = vec![2];
        assert_eq!(Solution::max_product(input), 2);

        let input = vec![2, 2];
        assert_eq!(Solution::max_product(input), 4);

        let input = vec![2, 2, 2, 2, 2];
        assert_eq!(Solution::max_product(input), 32);

        let input = vec![-1, -1];
        assert_eq!(Solution::max_product(input), 1);

        let input = vec![0, 2];
        assert_eq!(Solution::max_product(input), 2);
    }
}
