pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: [usize; 9] = [0; 9];
        let mut columns: [usize; 9] = [0; 9];
        let mut sub_boards: [usize; 9] = [0; 9];

        for i in 0..9 {
            for j in 0..9 {
                let box_index = (i / 3) * 3 + (j / 3);
                let flag = {
                    if board[i][j] != '.' {
                        1 << (board[i][j] as usize - 49)
                    } else {
                        0
                    }
                };

                // Check the submatrix for duplicate numbers
                if (sub_boards[box_index] & flag) > 0 {
                    return false;
                } else {
                    sub_boards[box_index] |= flag;
                }

                // Check the rows for duplicate numbers
                if (rows[i] & flag) > 0 {
                    return false;
                } else {
                    rows[i] |= flag;
                }

                // Check the columns for duplicate numbers
                if (columns[j] & flag) > 0 {
                    return false;
                } else {
                    columns[j] |= flag;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {

    use crate::Solution;

    #[test]
    fn it_works() {
        let board = vec![
            vec!['5','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9']
        ];

        let r = Solution::is_valid_sudoku(board);
        assert_eq!(r, true);

        let board = vec![
            vec!['8','3','.','.','7','.','.','.','.'], 
            vec!['6','.','.','1','9','5','.','.','.'], 
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'], 
            vec!['.','.','.','.','8','.','.','7','9']
        ];

        let r = Solution::is_valid_sudoku(board);
        assert_eq!(r, false);
    }
}
