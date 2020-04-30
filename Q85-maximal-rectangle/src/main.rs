use std::cmp::{min, max};

struct Solution {}

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        // solution 1
        // if matrix.is_empty() {
        //     return 0;
        // }
        //
        // let (m, n) = (matrix.len(), matrix[0].len());
        // let mut dp = vec![vec![0; n]; m];
        // let mut max_area = 0;
        //
        // for i in 0..m {
        //     for j in 0..n {
        //         if matrix[i][j] == '0' {
        //             continue;
        //         }
        //
        //         // find width of consecutive zero
        //         // for example, a row[i] is 0 0 1 1 1 0 1 0 then dp[i] is 0 0 1 2 3 0 1 0
        //         dp[i][j] = if j > 0 { dp[i][j - 1] + 1 } else { 1 };
        //         let mut width = dp[i][j];
        //
        //         for k in (0..=i).rev() {
        //             width = min(width, dp[k][j]);
        //             max_area = max(max_area, width * (i - k + 1) as i32);
        //         }
        //     }
        // }
        //
        //
        // max_area

        // solution 2
        // if matrix.is_empty() {
        //     return 0;
        // }
        //
        // let mut max_area = 0;
        // let (m, n) = (matrix.len(), matrix[0].len());
        // let mut vertical_consecutive_zeros = vec![0; n];
        //
        // for i in 0..m {
        //     for j in 0..n {
        //         vertical_consecutive_zeros[j] = if matrix[i][j] == '1' {
        //             vertical_consecutive_zeros[j] + 1
        //         } else {
        //             0
        //         };
        //     }
        //
        //     max_area = max(max_area, Solution::solution2_helper(&vertical_consecutive_zeros));
        // }
        //
        // max_area


        // solution 3
        if matrix.is_empty() {
            return 0;
        }

        let mut max_area = 0;
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut left = vec![0; n];
        let mut right = vec![n; n];
        let mut height = vec![0; n];

        for i in 0..m {
            let (mut cur_left, mut cur_right) = (0, n);

            for j in 0..n {
                height[j] = if matrix[i][j] == '1' { height[j] + 1 } else { 0 };
            }

            for j in 0..n {
                if matrix[i][j] == '1' {
                    left[j] = max(left[j], cur_left);
                } else {
                    left[j] = 0;
                    cur_left = j + 1;
                }
            }

            for j in (0..n).rev() {
                if matrix[i][j] == '1' {
                    right[j] = min(right[j], cur_right);
                } else {
                    right[j] = n;
                    cur_right = j;
                }
            }

            for j in 0..n {
                let (l, r) = (left[j] as i32, right[j] as i32);
                max_area = max(max_area, height[j] * (r - l));
            }
        }

        max_area
    }

    fn solution2_helper(heights: &Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut fixed_heights = vec![0];
        let mut max_area = 0;

        fixed_heights.extend(heights);
        fixed_heights.push(0);

        for i in 0..fixed_heights.len() {
            while !stack.is_empty() && fixed_heights[stack[stack.len() - 1]] > fixed_heights[i] {
                let last = stack.pop().unwrap();
                let width = (i - stack[stack.len() - 1] - 1) as i32;

                max_area = max_area.max(width * fixed_heights[last]);
            }

            stack.push(i);
        }

        max_area
    }
}

fn main() {
    let input = vec![
        vec!['1', '0', '1', '0', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '1', '1', '1', '1'],
        vec!['1', '0', '0', '1', '0'],
    ];
    assert_eq!(6, Solution::maximal_rectangle(input));
}
