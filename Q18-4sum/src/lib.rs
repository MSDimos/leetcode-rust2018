pub struct Solution;

impl Solution {

    pub fn three_sum(nums: &Vec<i32>, start: usize, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let len = nums.len();
        let mut i: usize = start;
        let mut l = 0;
        let mut r = 0;
        let mut s = 0;

        while i < len - 1 {
            l = i + 1;
            r = len - 1;

            while l < r {
                s = nums[i] + nums[l] + nums[r];

                if s == target {
                    // 去重
                    while l < r && nums[l] == nums[l + 1] {
                        l += 1;
                    }

                    while l < r && nums[r] == nums[r - 1] {
                        r -= 1;
                    }

                    res.push(vec![nums[i], nums[l], nums[r]]);
                    l += 1;
                } else if s > target {
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

    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort();

        let mut result = vec![];
        let mut idx = 0;
        let len = nums.len();

        if len < 4 {
            return result;
        }

        while idx < len - 3 {
            let mut res = Solution::three_sum(&nums, idx + 1, target - nums[idx]);

            for mut v in res {
                v.insert(0, nums[idx]);
                result.push(v);
            }

            while idx < len - 1 {
                if nums[idx] == nums[idx + 1] {
                    idx += 1;
                } else {
                    idx += 1;
                    break;
                }
            }
        }

        result
    }
}