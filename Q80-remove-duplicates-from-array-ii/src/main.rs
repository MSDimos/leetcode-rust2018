struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }

        let mut i = 0;
        let mut count = 1;
        const REPEAT_TIMES: usize = 2;

        for j in 1..nums.len() {
             if nums[i] != nums[j] {
                 i += 1;
                 nums[i] = nums[j];
                 count = 1;
             } else if count < REPEAT_TIMES {
                 i += 1;
                 nums[i] = nums[j];
                 count += 1;
             }
        }

        i as i32 + 1
    }
}

fn main() {
    let mut input = vec![1, 1, 1, 2, 2, 3];
    assert_eq!(5, Solution::remove_duplicates(&mut input));

    let mut input = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
    assert_eq!(7, Solution::remove_duplicates(&mut input));

    let mut input = vec![1, 2, 2];
    assert_eq!(3, Solution::remove_duplicates(&mut input));
}
