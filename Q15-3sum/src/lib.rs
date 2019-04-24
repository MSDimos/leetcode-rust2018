pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let len = nums.len();
        let mut i: usize = 0;
        let mut l = 0;
        let mut r = 0;
        let mut s = 0;
        nums.sort();

        if len < 3 {
            return vec![];
        }

        while i < len - 1 {
            l = i + 1;
            r = len - 1;

            while l < r {
                s = nums[i] + nums[l] + nums[r];

                if s == 0 {
                    // 去重
                    while l < r && nums[l] == nums[l + 1] {
                        l += 1;
                    }

                    while l < r && nums[r] == nums[r - 1] {
                        r -= 1;
                    }

                    res.push(vec![nums[i], nums[l], nums[r]]);
                    l += 1;
                } else if s > 0 {
                    r -= 1;
                } else {
                    l += 1;
                }
            }

            while i < len - 1 {
                if nums[i] == nums[i + 1] {
                    i += 1;
                } else {
                    i += 1;
                    break;
                }
            }
        }

        res
    }
}