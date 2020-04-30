struct Solution {}

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }
        let (m, n) = (grid.len(), grid[0].len());
        let mut path_sums = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                if i == 0 && j == 0 {
                    path_sums[i][j] = grid[i][j];
                } else if i == 0 && j > 0 {
                    path_sums[i][j] = grid[i][j] + path_sums[i][j - 1];
                } else if i > 0 && j == 0 {
                    path_sums[i][j] = grid[i][j] + path_sums[i - 1][j];
                } else {
                    path_sums[i][j] = path_sums[i - 1][j].min(path_sums[i][j - 1]) + grid[i][j];
                }
            }
        }

        path_sums[m - 1][n - 1]
    }
}

fn main() {
    assert_eq!(
        7,
        Solution::min_path_sum(vec![
            vec![1, 3, 1],
            vec![1, 5, 1],
            vec![4, 2, 1],
        ])
    );
}
