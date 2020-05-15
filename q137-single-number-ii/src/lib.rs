pub struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut a = 0;
        let mut b = 0;

        for num in nums {
            b = (b ^ num) & !a;
            a = (a ^ num) & !b;
        }

        return b;
    }

    pub fn single_number_s2(nums: Vec<i32>) -> i32 {
        let mut q0 = 0;
        let mut q1 = 0;

        for x in nums {
            let q0_next = ((!x) & (!q1) & q0) | (x & (!q1) & (!q0));
            let q1_next = ((!x) & q1 & (!q0)) | (x & (!q1) & q0);

            q0 = q0_next;
            q1 = q1_next;
        }

        q0
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::single_number(vec![2, 2, 3, 2]), 3);
        assert_eq!(Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);
    }

    #[test]
    fn it_works_s2() {
        assert_eq!(Solution::single_number_s2(vec![2, 2, 2, 3]), 3);
        assert_eq!(Solution::single_number_s2(vec![0, 1, 0, 1, 0, 1, 99]), 99);
    }
}
