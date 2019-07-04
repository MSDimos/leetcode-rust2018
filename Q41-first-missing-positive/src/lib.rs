pub struct Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut v: Vec<bool> = vec![false; len];

        for num in nums {
            if num <= 0 || num > (len as i32) {
                continue;
            } else {
                v[num as usize - 1] = true;
            }
        }

        let r =  v
            .into_iter()
            .enumerate()
            .find(|(idx, val)| val == &false)
            .unwrap_or((len, false));

        (r.0 + 1) as i32
    }
}

#[cfg(test)]
mod tests {

    use crate::Solution;

    #[test]
    fn it_works() {
        let v = Solution::first_missing_positive(vec![3, 4, -1, 1]);
        assert_eq!(v, 2);

        let v = Solution::first_missing_positive(vec![1, 2, 0]);
        assert_eq!(v, 3);

        let v = Solution::first_missing_positive(vec![7, 8, 9, 11, 12]);
        assert_eq!(v, 1);

        let v = Solution::first_missing_positive(vec![1]);
        assert_eq!(v, 2);

        let v = Solution::first_missing_positive(vec![]);
        assert_eq!(v, 1);
    }
}
