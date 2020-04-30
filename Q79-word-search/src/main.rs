struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        if word.is_empty() {
            return false;
        }

        let (m, n) = (board.len(), board[0].len());
        let word_bytes = word.into_bytes();
        let mut marks = vec![vec![false; n]; m];

        for i in 0..m {
            for j in 0..n {
                if board[i][j] == word_bytes[0] as char && !marks[i][j] {
                    marks[i][j] = true;

                    if Solution::next(&board, &mut marks, &word_bytes, 1, (i, j)) {
                        return true;
                    }

                    marks[i][j] = false;
                }
            }
        }

        false
    }

    fn get_coords((i, j): (usize, usize), (m, n): (usize, usize)) -> Vec<(usize, usize)> {
        let i = i as isize;
        let j = j as isize;
        let mut _coords = vec![(i, j - 1), (i, j + 1), (i - 1, j), (i + 1, j)];
        let mut coords = vec![];

        for coord in _coords {
            if coord.0 >= 0 && coord.0 < m as isize && coord.1 >= 0 && coord.1 < n as isize {
                coords.push((coord.0 as usize, coord.1 as usize));
            }
        }

        coords
    }

    fn next(
        board: &Vec<Vec<char>>, marks: &mut Vec<Vec<bool>>,
        word_bytes: &Vec<u8>, idx: usize, (i, j): (usize, usize),
    ) -> bool {
        if idx == word_bytes.len() {
            return true;
        }

        let mut coords = Solution::get_coords((i, j), (board.len(), board[0].len()));

        for coord in coords {
            if !marks[coord.0][coord.1] && word_bytes[idx] as char == board[coord.0][coord.1] {
                marks[coord.0][coord.1] = true;

                if Solution::next(board, marks, word_bytes, idx + 1, coord) {
                    return true;
                }

                marks[coord.0][coord.1] = false;
            }
        }

        false
    }
}

fn main() {
    let input = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    assert_eq!(true, Solution::exist(input.clone(), "ABCCED".to_string()));
    assert_eq!(true, Solution::exist(input.clone(), "SEE".to_string()));
    assert_eq!(false, Solution::exist(input.clone(), "ABCB".to_string()));

    let input = vec![
        vec!['C','A','A'],
        vec!['A','A','A'],
        vec!['B','C','D'],
    ];
    assert_eq!(true, Solution::exist(input.clone(), "AAB".to_string()));
    assert_eq!(true, Solution::exist(input.clone(), "AAAAA".to_string()));
}
