struct Solution {}

impl Solution {
    #[inline]
    fn swap(nums: &mut Vec<i32>, i: usize, j: usize) {
        let temp = nums[i];

        nums[i] = nums[j];
        nums[j] = temp;
    }

    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut white_ptr, mut blue_ptr) = (0, nums.len() - 1);
        let mut current_ptr = 0;

        while current_ptr <= blue_ptr {
            if nums[current_ptr] == 0 {
                Self::swap(nums, current_ptr, white_ptr);
                // Since current_ptr if from left to right,
                // so 0..current_ptr must meet the the condition that all numbers are <= 1
                // so both white_ptr and current_ptr should go forward
                white_ptr += 1;
                current_ptr += 1;
            } else if nums[current_ptr] == 2 {
                Self::swap(nums, current_ptr, blue_ptr);
                // differing from above, current_ptr..blue_ptr can't guarantee that all numbers are 1
                // nums[current_ptr] after swapping is possibly each one of 0, 1, 2
                if blue_ptr > 0 {
                    blue_ptr -= 1;
                } else {
                    // if blue_ptr is 0, blue_ptr - 1 will cause runtime error about overflow.
                    break;
                }
            } else {
                current_ptr += 1;
            }
        }
    }
}

fn main() {
    let mut input = vec![2, 0, 2, 1, 1, 0];

    Solution::sort_colors(&mut input);
    assert_eq!(vec![0, 0, 1, 1, 2, 2], input);

    let mut input = vec![2];

    Solution::sort_colors(&mut input);
    assert_eq!(vec![2], input);

    let mut input = vec![1];

    Solution::sort_colors(&mut input);
    assert_eq!(vec![1], input);

    let mut input = vec![0];

    Solution::sort_colors(&mut input);
    assert_eq!(vec![0], input);
}
