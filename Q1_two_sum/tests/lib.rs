extern crate Q1_two_sum;

use crate::Q1_two_sum::Solution;

#[test]
fn add_sum() {
    let nums = vec![3, 2 ,4];
    let target = 6;

    assert_eq!(Solution::two_sum(nums, target), vec![1 ,2]);
}