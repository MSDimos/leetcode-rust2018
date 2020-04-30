struct Solution {}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        assert!(m <= 100, "m >100");
        assert!(n <= 100, "n > 100");

        if m == 0 || n == 0 {
            return 1;
        }

        let mut dp = [[0; 100]; 100];
        dp[0][0] = 1;

        for i in 0..m {
            for j in 0..n {
                let _i = i as usize;
                let _j = j as usize;

                if i == 0 || j == 0 {
                    dp[_i][_j] = 1;
                } else {
                    dp[_i][_j] = dp[_i - 1][_j] + dp[_i][_j - 1];
                }
            }
        }

        return dp[(m - 1) as usize][(n - 1) as usize];
    }
}

fn main() {
    println!("{}", Solution::unique_paths(3, 2));
    println!("{}", Solution::unique_paths(7, 3));
    println!("{}", Solution::unique_paths(23, 12));
}