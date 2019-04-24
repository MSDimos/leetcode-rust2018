pub struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        // 排序数组，方便去重
        nums.sort();
        let len = nums.len();

        if len < 3 {
            return 0;
        }

        let mut res = 0;
        let mut idx = 0;
        let mut l;
        let mut r;
        let mut d = 2147483647;

        while idx < len - 1 {
            l = idx + 1;
            r = len - 1;

            while l < r {
                let sum = nums[idx] + nums[l] + nums[r];
                let signed_d = sum - target;
                let temp_d = (sum - target).abs();

                if temp_d < d {
                    d = temp_d;
                    res = sum;
                }

                if signed_d == 0 {
                    return target;
                } else if signed_d > 0 {
                    r -= 1;
                } else {
                    l += 1;
                }
            }

            // 向前移动，去掉重复的nums[idx]，避免重复的不必要的运算
            while idx < len - 1 {
                if nums[idx] == nums[idx + 1] {
                    idx += 1;
                } else {
                    idx += 1;
                    break;
                }
            }
        }


        res

    }
}