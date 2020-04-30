struct Solution {}

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        if matrix.is_empty() {
            return;
        }

        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut fisrt_row, mut first_column) = (false, false);

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    if i == 0 || j == 0 {
                        fisrt_row = i == 0 || fisrt_row;
                        first_column = j == 0 || first_column;
                    }

                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        for i in 1..m {
            if matrix[i][0] == 0 {
                for j in 0..n {
                    matrix[i][j] = 0;
                }
            }
        }

        for j in 1..n {
            if matrix[0][j] == 0 {
                for i in 0..m {
                    matrix[i][j] = 0;
                }
            }
        }

        if fisrt_row {
            for j in 0..n {
                matrix[0][j] = 0;
            }
        }

        if first_column {
            for i in 0..m {
                matrix[i][0] = 0;
            }
        }
    }
}

fn main() {
    let mut input = vec![
        vec![1, 1, 1],
        vec![1, 0, 1],
        vec![1, 1, 1],
    ];
    Solution::set_zeroes(&mut input);
    let output = vec![
        vec![1, 0, 1],
        vec![0, 0, 0],
        vec![1, 0, 1],
    ];

    assert_eq!(output, input);


    let mut input = vec![
        vec![1, 0, 1],
        vec![1, 0, 1],
        vec![1, 1, 1],
    ];
    Solution::set_zeroes(&mut input);
    let output = vec![
        vec![0, 0, 0],
        vec![0, 0, 0],
        vec![1, 0, 1],
    ];

    assert_eq!(output, input);


    let mut input = vec![
        vec![0, 0, 0, 5],
        vec![4, 3, 1, 4],
        vec![0, 1, 1, 4],
        vec![1, 2, 1, 3],
        vec![0, 0, 1, 1]
    ];
    Solution::set_zeroes(&mut input);
    let output = vec![
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 4],
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 3],
        vec![0, 0, 0, 0],
    ];

    assert_eq!(output, input);


    let mut input = vec![
        vec![3, 5, 5, 6, 9, 1, 4, 5, 0, 5],
        vec![2, 7, 9, 5, 9, 5, 4, 9, 6, 8],
        vec![6, 0, 7, 8, 1, 0, 1, 6, 8, 1],
        vec![7, 2, 6, 5, 8, 5, 6, 5, 0, 6],
        vec![2, 3, 3, 1, 0, 4, 6, 5, 3, 5],
        vec![5, 9, 7, 3, 8, 8, 5, 1, 4, 3],
        vec![2, 4, 7, 9, 9, 8, 4, 7, 3, 7],
        vec![3, 5, 2, 8, 8, 2, 2, 4, 9, 8]
    ];
    Solution::set_zeroes(&mut input);
    let output = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![2, 0, 9, 5, 0, 0, 4, 9, 0, 8],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![5, 0, 7, 3, 0, 0, 5, 1, 0, 3],
        vec![2, 0, 7, 9, 0, 0, 4, 7, 0, 7],
        vec![3, 0, 2, 8, 0, 0, 2, 4, 0, 8]
    ];
    assert_eq!(output, input);
}
