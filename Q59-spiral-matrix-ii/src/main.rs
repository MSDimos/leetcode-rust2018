struct Solution {}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let cycle_num = ((n + 1) / 2) as usize;
        let mut matrix = vec![vec![-1; n]; n];
        let mut cycle = 0;
        let mut count = 0;

        while cycle < cycle_num {
            // top line
            for j in cycle..(n - cycle) {
                count += 1;
                matrix[cycle][j] = count;
            }

            // right line
            for i in (cycle + 1)..(n - cycle) {
                count += 1;
                matrix[i][n - cycle - 1] = count;
            }

            // bottom line
            for j in (cycle..(n - cycle - 1)).rev() {
                // avoid assigning twice
                if n - cycle - 1 != cycle {
                    count += 1;
                    matrix[n - cycle - 1][j] = count;
                }
            }

            //left line
            for i in ((cycle + 1)..(n - cycle -1)).rev() {
                // avoid assigning twice
                if n - cycle - 1 != cycle {
                    count += 1;
                    matrix[i][cycle] = count;
                }
            }

            cycle += 1;
        }

        matrix
    }
}

fn main() {
    assert_eq!(
        vec![
            vec![1, 2, 3],
            vec![8, 9, 4],
            vec![7, 6, 5],
        ],
        Solution::generate_matrix(3)
    );
}
