struct Solution {}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_pos = 0;
        let mut end_pos = 0;
        let len = nums.len() as i32;

        for (idx, num) in nums.into_iter().enumerate() {
            max_pos = max_pos.max(num + idx as i32);
            max_pos = max_pos.min(len - 1);

            if idx == end_pos as usize  {
                // update end_pos as max_pos
                end_pos = max_pos;
            }
        }

        end_pos == len - 1
    }
}

fn main() {
    assert_eq!(true, Solution::can_jump(vec![2, 3, 1, 1, 4]));
    assert_eq!(false, Solution::can_jump(vec![3, 2, 1, 0, 4]));
}
