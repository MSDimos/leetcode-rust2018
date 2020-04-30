struct Solution {}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        // 1. dichotomy
        // if x < 2 {
        //     return x;
        // } else {
        //     let (mut left, mut right) = (2, x / 2);
        //
        //     while left <= right {
        //         let pivot = left + (right - left) / 2;
        //         let square = pivot * pivot;
        //
        //         if square > x {
        //             right = pivot - 1;
        //         } else if square < x {
        //             left = pivot + 1;
        //         } else {
        //             return pivot;
        //         }
        //     }
        //
        //     return right;
        // }

        // 2. Newton's method
        if x < 2 {
            return x;
        } else {
            let x = x as f64;
            let mut x0 = x as f64;
            let mut x1 = ((x0 + x / x0) / 2.0) as f64;
            while (x0 - x1).abs() >= 1.0 {
                x0 = x1;
                x1 = (x0 + x / x0) / 2.0;
            }
            return x1 as i32;
        }

        // 3. amazing constant
        // Google for 0x5fe6ec85e7de30da!!!!!
    }
}

fn main() {
    assert_eq!(2, Solution::my_sqrt(4));
    assert_eq!(2, Solution::my_sqrt(8));
    assert_eq!(11111, Solution::my_sqrt(123456789));
}
