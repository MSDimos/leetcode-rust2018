use std::cmp::Ordering;

pub struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = (left + right) / 2;

            match nums[mid].cmp(&nums[right]) {
                Ordering::Greater => left = mid + 1,
                Ordering::Less => right = mid,
                Ordering::Equal => right -= 1,
            }
        }

        nums[left]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let input = vec![1, 3, 5];
        assert_eq!(Solution::find_min(input), 1);

        let input = vec![2, 2, 2, 0, 1];
        assert_eq!(Solution::find_min(input), 0);

        let input = vec![2, 2];
        assert_eq!(Solution::find_min(input), 2);
    }
}
