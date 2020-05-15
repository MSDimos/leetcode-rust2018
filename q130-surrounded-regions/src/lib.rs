use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if !board.is_empty() {
            let row = board.len() as i32;
            let col = board[0].len() as i32;
            let mut not_surrounded_items = HashSet::new();

            // // left and right edges
            for i in 0..row {
                Self::find_not_surrounded_items(board, (i, 0), &mut not_surrounded_items);
                Self::find_not_surrounded_items(board, (i, col - 1), &mut not_surrounded_items);
            }

            // top and bottom edges
            for j in 0..col {
                Self::find_not_surrounded_items(board, (0, j), &mut not_surrounded_items);
                Self::find_not_surrounded_items(board, (row - 1, j), &mut not_surrounded_items);
            }

            for i in 0..row {
                for j in 0..col {
                    let (i, j) = (i as usize, j as usize);

                    if board[i][j] == 'O' && !not_surrounded_items.contains(&(i, j)) {
                        board[i][j] = 'X';
                    }
                }
            }
        }
    }

    fn find_not_surrounded_items(
        board: &[Vec<char>],
        coordinate: (i32, i32),
        not_surrounded_items: &mut HashSet<(usize, usize)>,
    ) {
        let (i, j) = coordinate;

        if i < 0
            || j < 0
            || i >= board.len() as i32
            || j >= board[0].len() as i32
            || board[i as usize][j as usize] != 'O'
            || not_surrounded_items.contains(&(i as usize, j as usize))
        {
        } else {
            not_surrounded_items.insert((i as usize, j as usize));
            Self::find_not_surrounded_items(board, (i - 1, j), not_surrounded_items);
            Self::find_not_surrounded_items(board, (i + 1, j), not_surrounded_items);
            Self::find_not_surrounded_items(board, (i, j - 1), not_surrounded_items);
            Self::find_not_surrounded_items(board, (i, j + 1), not_surrounded_items);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let mut input = vec![
            vec!['X'; 4],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut input);
        assert_eq!(
            input,
            vec![
                vec!['X'; 4],
                vec!['X'; 4],
                vec!['X'; 4],
                vec!['X', 'O', 'X', 'X'],
            ]
        );

        let mut input = vec![vec!['O'; 3]; 3];
        let mut output = vec![vec!['O'; 3]; 3];
        Solution::solve(&mut input);
        assert_eq!(input, output);

        let mut input = vec![
            vec!['O', 'X', 'X', 'O', 'X'],
            vec!['X', 'O', 'O', 'X', 'O'],
            vec!['X', 'O', 'X', 'O', 'X'],
            vec!['O', 'O', 'O', 'O', 'O'],
            vec!['X', 'O', 'O', 'X', 'O'],
        ];
        let output = vec![
            vec!['O', 'X', 'X', 'O', 'X'],
            vec!['X', 'O', 'O', 'X', 'O'],
            vec!['X', 'O', 'X', 'O', 'X'],
            vec!['O', 'O', 'O', 'O', 'O'],
            vec!['X', 'O', 'O', 'X', 'O'],
        ];
        Solution::solve(&mut input);
        assert_eq!(input, output);
    }
}
