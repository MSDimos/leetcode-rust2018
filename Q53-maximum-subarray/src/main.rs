struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // O(n)
        if nums.is_empty() {
            return 0;
        } else {
            let mut result = nums[0];
            let mut dp = vec![nums[0]];

            for i in 1..nums.len() {
                dp.push(i32::min_value());
            }

            for i in 1..nums.len() {
                dp[i] = nums[i].max(dp[i - 1] + nums[i]);
                result = result.max(dp[i]);
            }

            return result;
        }
    }
}

fn main() {
    assert_eq!(6, Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]));
    assert_eq!(1, Solution::max_sub_array(vec![1]));
    assert_eq!(0, Solution::max_sub_array(vec![]));
}
