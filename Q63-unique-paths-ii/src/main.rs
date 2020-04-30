struct Solution {}

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid.is_empty() {
            return 0
        }

        let mut steps: [[i32; 100]; 100] = [[0; 100]; 100];
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());

        for i in 0..m {
            for j in 0..n {
                // if encounter an obstacles, skip
                if obstacle_grid[i][j] == 1 {
                    continue;
                }

                if i == 0 && j == 0 {
                    steps[i][j] = 1;
                } else if i == 0 && j > 0 {
                    steps[i][j] = steps[i][j - 1];
                } else if j == 0 && i > 0 {
                    steps[i][j] = steps[i - 1][j];
                } else {
                    steps[i][j] = steps[i - 1][j] + steps[i][j - 1];
                }
            }
        }

        steps[m - 1][n - 1]
    }
}

fn main() {
    let input = vec![
        vec![0, 0, 0],
        vec![0, 1, 0],
        vec![0, 0, 0],
    ];
    assert_eq!(2, Solution::unique_paths_with_obstacles(input));

    let input = vec![vec![1]];
    assert_eq!(0, Solution::unique_paths_with_obstacles(input));


    let input = vec![vec![1, 0]];
    assert_eq!(0, Solution::unique_paths_with_obstacles(input));

    let input = vec![
        vec![1, 0, 0],
        vec![0, 0, 0],
        vec![0, 0, 0],
    ];
    assert_eq!(0, Solution::unique_paths_with_obstacles(input));
}
