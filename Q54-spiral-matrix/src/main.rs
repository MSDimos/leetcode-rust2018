struct Solution {}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() {
            return vec![];
        }

        let m = matrix.len();
        let n = matrix[0].len();
        // number of cycle
        let c = (m.min(n) + 1) / 2;
        let mut result = vec![];
        let mut k = 0;

        while k < c {
            // reading order is a rectangle

            // scan top line from left to right
            for j in k..(n - k) {
                result.push(matrix[k][j]);
            }
            // scan right line from top to bottom
            for i in (k + 1)..(m - k) {
                result.push(matrix[i][n - k - 1]);
            }
            // scan bottom line from right to left
            for j in (k..(n - k - 1)).rev() {
                if m - k - 1 != k {
                    result.push(matrix[m - k - 1][j]);
                }
            }
            // scan left line from bottom to top
            for i in ((k + 1)..(m - k - 1)).rev() {
                if n - k - 1 != k {
                    result.push(matrix[i][k]);
                }
            }

            k += 1;
        }

        result
    }
}


fn main() {
    assert_eq!(
        vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
        Solution::spiral_order(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ])
    );
}
