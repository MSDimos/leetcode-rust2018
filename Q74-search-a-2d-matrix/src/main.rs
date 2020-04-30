struct Solution {}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() {
            return false;
        }

        // Using 1 instead of 0 to prevent subtracting with overflow
        let (mut row, mut column) = (1, matrix[0].len());

        while row <= matrix.len() && column >= 1 {
            if matrix[row - 1][column - 1] < target {
                row += 1;
            } else if matrix[row - 1][column - 1] > target {
                column -= 1;
            } else {
                return true;
            }
        }

        false
    }
}

fn main() {
    assert_eq!(true, Solution::search_matrix(vec![
        vec![1, 3, 5, 7],
        vec![10, 11, 16, 20],
        vec![23, 30, 34, 50],
    ], 3));

    assert_eq!(false, Solution::search_matrix(vec![
        vec![1, 3, 5, 7],
        vec![10, 11, 16, 20],
        vec![23, 30, 34, 50],
    ], 13));
}
