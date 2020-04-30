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
    pub fn total_n_queens(n: i32) -> i32 {
        let mut result = 0;

        Solution::backtrack(0, n as usize, &mut result);
        result
    }

    pub fn backtrack(i: usize, n: usize, result: &mut i32) {
        if i == n {
            *result += 1;
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
    let res = Solution::total_n_queens(4);
    println!("{:#?}", res);
}
