pub struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = (left + right) / 2;

            if nums[mid] < nums[right] {
                right = mid;
            } else {
                left = mid + 1;
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
        let input = vec![3, 4, 5, 1, 2];
        assert_eq!(Solution::find_min(input), 1);

        let input = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(Solution::find_min(input), 0);

        let input = vec![1, 2];
        assert_eq!(Solution::find_min(input), 1);

        let input = vec![2, 1];
        assert_eq!(Solution::find_min(input), 1);

        let input = vec![1, 2, 3, 4];
        assert_eq!(Solution::find_min(input), 1);
    }
}
