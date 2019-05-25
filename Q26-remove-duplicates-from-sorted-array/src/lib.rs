pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup_by(|a, b| a == b);

        nums.len() as i32
    }
}