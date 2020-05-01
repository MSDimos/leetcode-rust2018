pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid = (left + right) / 2;

            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] > target {
                if mid > 0 {
                    right = mid - 1;
                } else {
                    return 0;
                }
            } else if nums[mid] < target {
                if mid < nums.len() - 1 {
                    left = mid + 1;
                } else {
                    return nums.len() as i32;
                }
            }
        }

        return left as i32;
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn it_works() {
        let r = Solution::search_insert(vec![1, 3, 5, 6], 5);
        assert_eq!(r, 2);
    }
}
