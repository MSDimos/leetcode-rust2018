pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }

        let rotated_index = Solution::find_rotated_index(&nums, 0, nums.len() - 1);

        if let Some(ri) = rotated_index {
            // ri处的值不参与二分查找
            if ri < nums.len() && target == nums[ri] {
                ri as i32
            } else if target <= nums[nums.len() - 1] {
                Solution::find_target(&nums, ri + 1, nums.len() - 1, target)
            } else {
                Solution::find_target(&nums, 0, ri - 1, target)
            }
        } else {
            // 不存在旋转点，直接二分查找
            Solution::find_target(&nums, 0, nums.len() - 1, target)
        }
    }

    pub fn find_target(nums: &[i32], left: usize, right: usize, target: i32) -> i32 {
        if left < right && right < nums.len() {
            let mid = (left + right) / 2;
            let left_part = Solution::find_target(&nums, left, mid, target);
            let right_part = Solution::find_target(&nums, mid + 1, right, target);

            if left_part == -1 {
                right_part
            } else {
                left_part
            }
        } else if left == right && right < nums.len() && nums[left] == target {
            left as i32
        } else {
            -1
        }
    }

    pub fn find_rotated_index(nums: &[i32], left: usize, right: usize) -> Option<usize> {
        if left < right && right < nums.len() {
            if nums[left] < nums[right] {
                // 单调递增，不在这个区间
                None
            } else {
                // 不是严格单调递增
                let mid = (left + right) / 2;
                let left_part = Solution::find_rotated_index(&nums, left, mid);
                let right_part = Solution::find_rotated_index(&nums, mid + 1, right);
                let full = left_part.or(right_part);

                if full.is_none() {
                    // 左右两段都是单调递增的，所以h旋转点就在这里
                    Some(mid + 1)
                } else {
                    // 分两段必有一段是单调递增的
                    full
                }
            }
        } else {
            None
        }
    }
}


#[cfg(test)]
mod test {

    use crate::Solution;

    #[test]
    fn find_rotated_index() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let i = Solution::search(nums, 0);
        assert_eq!(i, 4);

        let nums = vec![5, 1, 3];
        let i = Solution::search(nums.clone(), 1);
        assert_eq!(i, 1);

        let i = Solution::search(nums.clone(), 3);
        assert_eq!(i, 2);
    }
}