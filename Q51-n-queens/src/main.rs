static mut LINES: [usize; 128] = [0; 128];
static mut COLUMNS: [bool; 128] = [false; 128];
// The diagonal corresponding to the equation: y = -x +/- constant
// first usize is used for situation if i >= j
// second usize if used for situation if i < j
static mut POSITIVE_DIAGONAL: [(bool, bool); 128] = [(false, false); 128];
// The diagonal corresponding to the equation: y = x +/- constant
static mut NEGATIVE_DIAGONAL: [bool; 255] = [false; 255];

struct Solution {}

impl Solution {

    fn push_result(result: &mut Vec<Vec<String>>, n: usize) {
        let mut board = vec![];

        for k in 0..n {
            unsafe {
                let lz = LINES[k].trailing_zeros() as usize;
                let mut l = String::from(".".repeat(lz));

                l.push('Q');
                l.push_str(&".".repeat(n - lz - 1));
                board.push(l);
            }
        }

        result.push(board);
    }

    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = vec![];

        Solution::backtrack(0, n as usize, &mut result);
        result
    }

    pub fn backtrack(i: usize, n: usize, result: &mut Vec<Vec<String>>) {
        if i == n {
            Solution::push_result(result, n);
        } else {
            for j in 0..n {
                if Solution::check_board(i, j) {
                    unsafe {
                        LINES[i] = 0b0001 << j;
                        COLUMNS[j] = true;
                        if i >= j {
                            POSITIVE_DIAGONAL[i - j].0 = true;
                        } else {
                            POSITIVE_DIAGONAL[j - i].1 = true;
                        }
                        NEGATIVE_DIAGONAL[i + j] = true;
                    }

                    Solution::backtrack(i + 1, n, result);
                    unsafe {
                        LINES[i] = 0b0000;
                        COLUMNS[j] = false;
                        if i >= j {
                            POSITIVE_DIAGONAL[i - j].0 = false;
                        } else {
                            POSITIVE_DIAGONAL[j - i].1 = false;
                        }
                        NEGATIVE_DIAGONAL[i + j] = false;
                    }
                }
            }
        }
    }

    #[inline]
    fn check_board(i: usize, j: usize) -> bool {
        unsafe {
            // no other queen in the line-i
            LINES[i] == 0 &&
                // no other queen in the column-i
                !COLUMNS[j] &&
                // no other queen in the negative diagonal that meet the condition 'i + j == constant'
                !NEGATIVE_DIAGONAL[i + j]&&
                // no other queen in the negative diagonal that meet the condition 'abs(i - j) == constant'
                ((i >= j && !POSITIVE_DIAGONAL[i - j].0) || j > i && !POSITIVE_DIAGONAL[j - i].1)
        }
    }
}

fn main() {
    let res = Solution::solve_n_queens(4);
    println!("{:#?}", res);
}
