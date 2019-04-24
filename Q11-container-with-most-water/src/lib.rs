pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, height.len() - 1);
        let mut max_area = 0;

        while left < right {
            max_area = max_area.max(height[left].min(height[right]) * ((right - left) as i32));
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_area
    }
}