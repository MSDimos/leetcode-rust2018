struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        let n = n as usize;

        dp[0] = 1;
        dp[1] = 2;

        for i in 2..n {
            dp[i] = dp[i - 1] + dp[i - 2];
        }

        dp[n - 1]
    }
}

fn main() {
    assert_eq!(2, Solution::climb_stairs(2));
    assert_eq!(3, Solution::climb_stairs(3));
    assert_eq!(1, Solution::climb_stairs(1));
}
