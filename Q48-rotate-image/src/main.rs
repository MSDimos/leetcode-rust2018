use std::mem::swap;

struct Solution {}

impl Solution {
    // rotating matrix 90 degrees clockwise is equivalent to two steps below:
    // 1. transpose the matrix
    // 2. flip horizontally
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        Solution::transpose(matrix);
        // println!("{:#?}", matrix);

        for i in 0..matrix.len() {
            matrix[i].reverse();
        }
    }

    fn transpose(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        for i in 0..n {
            for j in (i + 1)..n {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }
    }
}

fn main() {
    let mut matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    Solution::rotate(&mut matrix);

    assert_eq!(matrix, vec![
        vec![7, 4, 1],
        vec![8, 5, 2],
        vec![9, 6, 3],
    ]);

    let mut matrix = vec![
        vec![ 5, 1, 9,11],
        vec![ 2, 4, 8,10],
        vec![13, 3, 6, 7],
        vec![15,14,12,16],
    ];
    Solution::rotate(&mut matrix);
    assert_eq!(matrix, vec![
        vec![15,13, 2, 5],
        vec![14, 3, 4, 1],
        vec![12, 6, 8, 9],
        vec![16, 7,10,11],
    ]);
}
