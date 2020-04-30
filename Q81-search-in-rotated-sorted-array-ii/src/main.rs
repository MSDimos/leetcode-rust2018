struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums.is_empty() {
            return false;
        }

        for i in 1..nums.len() {
            if nums[i - 1] > nums[i] {
                if target >= nums[0] && target <= nums[i - 1] {
                    return Solution::binary_search(&nums, 0, i - 1, target);
                } else if target >= nums[i] && target <= nums[nums.len() - 1] {
                    return Solution::binary_search(&nums, i, nums.len() - 1, target);
                }
                break;
            }
        }

        Solution::binary_search(&nums, 0, nums.len() - 1, target)
    }

    fn binary_search(nums: &Vec<i32>, left: usize, right: usize, target: i32) -> bool {
        let mid = (left + right) / 2;

        if left == right {
            return nums[left] == target;
        }

        if target >= nums[left] && target <= nums[mid] {
            return Solution::binary_search(nums, left, mid, target);
        } else if target >= nums[mid + 1] && target <= nums[right] {
            return Solution::binary_search(nums, mid + 1, right, target);
        }

        false
    }
}

fn main() {
    assert_eq!(true, Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0));
    assert_eq!(false, Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3));
    assert_eq!(false, Solution::search(vec![], 3));
    assert_eq!(true, Solution::search(vec![3], 3));
    assert_eq!(true, Solution::search(vec![3, 3, 3], 3));
    assert_eq!(true, Solution::search(vec![1, 2, 3, 4, 5, 6], 3));
    assert_eq!(true, Solution::search(vec![1, 2, 3, 4, 5, 6], 1));
    assert_eq!(true, Solution::search(vec![1, 2, 3, 4, 5, 6], 6));
}
