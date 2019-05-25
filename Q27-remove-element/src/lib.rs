pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {

        let ptr = nums as *const Vec<i32> as *mut Vec<i32>;
        let pairs = unsafe {
            (&*ptr).iter().enumerate()
        };
        let mut n = nums.len();
        let mut i = 0;

        loop {
            if i < n {
                if nums[i] == val {
                    nums.remove(i);
                    n = nums.len();
                } else {
                    i += 1;
                }
            } else {
                break;
            }
        }

        nums.len() as i32
    }
}