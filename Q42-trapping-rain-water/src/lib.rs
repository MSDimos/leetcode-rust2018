use std::cmp::{max, min};

pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut stack: Vec<usize> = vec![];
        let len = height.len();

        // using stack
        // if element is not great than top element of stack, push it
        // if it's not, it means that bottom element and this element form a bucket.
        for i in 0..len {
            // calculate all buckets' sizes
            while !stack.is_empty() && height[i] > height[stack[stack.len() - 1]] {
                let top = stack.pop().unwrap();

                if !stack.is_empty() {
                    let cell_width = (i - stack[stack.len() - 1] - 1) as i32;
                    let cell_height = height[i].min(height[stack[stack.len() - 1]]) - height[top];
                    ans += cell_height * cell_width;
                } else {
                    break;
                }
            }

            stack.push(i);
        }

        ans
    }

    fn trap_s2(heights: Vec<i32>) -> i32 {
        let len = heights.len();
        let mut max_left = vec![0; len];
        let mut max_right = vec![0; len];
        let mut sum = 0;

        max_left[0] = heights[0];
        max_right[len - 1] = heights[len - 1];

        for i in 1..len {
            max_left[i] = max(max_left[i - 1], heights[i]);
            max_right[len - 1 - i] = max(max_right[len - i], heights[len - 1 - i]);
        }

        for i in 0..len {
            if min(max_left[i], max_right[i]) > heights[i] {
                sum += min(max_left[i], max_right[i]) - heights[i];
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn it_works() {
        let r = Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]);
        assert_eq!(r, 6);
    }

    #[test]
    fn it_works_s2() {
        let r = Solution::trap_s2(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]);
        assert_eq!(r, 6);
    }
}
