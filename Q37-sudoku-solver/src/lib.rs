pub struct Solution;

type VVc = Vec<Vec<char>>;
type SubBox = [usize; 9];

impl Solution {
    pub fn solve_sudoku(board: &mut VVc) {
        let mut rows: SubBox = [0; 9];
        let mut columns: SubBox = [0; 9];
        let mut boxs: SubBox = [0; 9];

        Solution::place_known_elements(board, &mut rows, &mut columns, &mut boxs);

        if !Solution::backtrace(board, 0, 0, &mut rows, &mut columns, &mut boxs) {
            panic!("This sudoku is unresolvable");
        }
    }

    // Place known elements
    pub fn place_known_elements(board: &mut VVc, rows: &mut SubBox, columns: &mut SubBox, boxs: &mut SubBox) {
        for i in 0..9 {
            for j in 0..9 {
                let flag = {
                    if board[i][j] != '.' {
                        1 << (board[i][j] as usize - 49)
                    } else {
                        0
                    }
                };
                rows[i] |= flag;
                columns[j] |= flag;
                boxs[(i / 3) * 3 + j / 3] |= flag;
            }
        }
    }

    #[inline]
    fn check(flag: usize, rows: usize, columns: usize, boxs: usize) -> bool {
        return !(rows| columns | boxs) & flag > 0;
    }

    fn backtrace(board: &mut VVc, mut i: usize, mut j: usize, rows: &mut SubBox, columns: &mut SubBox, boxs: &mut SubBox) -> bool {
        // progressive scanning until find an empty element
        while board[i][j] != '.' {
            j += 1;

            if j >= 9 {
                i += 1;
                j = 0;
            }

            if i >= 9 {
                return true;
            }
        }

        // calculate block index through index of element
        let block_index = i / 3 * 3 + j / 3;

        // backtrace through value of number
        for num in 0..9 {
            let flag = 1 << num;

            if Solution::check(flag, rows[i], columns[j], boxs[block_index]) {
                rows[i] |= flag;
                columns[j] |= flag;
                boxs[block_index] |= flag;
                board[i][j] = (flag.trailing_zeros() + 49) as u8 as char;

                if Solution::backtrace(board, i, j, rows, columns, boxs) {
                    return true;
                } else {
                    // backtrace
                    rows[i] &= !flag;
                    columns[j] &= !flag;
                    boxs[block_index] &= !flag;
                    board[i][j] = '.';
                }
            }
        }

        false
    }
}