pub struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }

        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut start = -1;
        let mut end = -1;

        while left <= right {
            let mid = (left + right) / 2;

            if nums[mid] == target {
                start = mid as i32;
                end = start;
                break;
            } else if mid < (nums.len() - 1) && nums[mid] < target {
                left = mid + 1;
            } else if mid > 0 && nums[mid] > target {
                right = mid - 1;
            } else {
                break;
            }
        }

        while start >= 0 && end < nums.len() as i32 {
            if start > 0 && nums[(start - 1) as usize] == target {
                start -= 1;
            } else if end < (nums.len() - 1) as i32 && nums[(end + 1) as usize] == target {
                end += 1;
            } else {
                break;
            }
        }

        return vec![start, end];
    }
}

#[cfg(test)]
mod tests {

    use crate::Solution;

    #[test]
    fn it_works() {
        let r = Solution::search_range(vec![5, 6, 7, 8, 8, 9], 0);
        assert_eq!(r, vec![3, 4]);
    }
}
