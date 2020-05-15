pub struct Solution {}

impl Solution {
    pub fn single_number(mut nums: Vec<i32>) -> i32 {
        for i in 1..nums.len() {
            nums[0] ^= nums[i];
        }

        nums[0]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
    }
}
